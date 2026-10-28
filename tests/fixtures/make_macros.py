"""Build tests/fixtures/macros.xlsm: a small macro-enabled workbook for the VBA
reader. openpyxl writes the sheets; the VBA project (xl/vbaProject.bin) is
written here from scratch, since openpyxl can only copy an existing one.

vbaProject.bin is a Compound File (MS-CFB) holding the VBA storage: the `dir`
stream lists the modules and each module stream holds its source, compressed
as MS-OVBA 2.4.1 describes. The project carries just what a reader needs to
list modules and read their source (no compiled code, no references).

    pip install openpyxl==3.1.5
    python tests/fixtures/make_macros.py
"""

import struct
import zipfile
from io import BytesIO
from pathlib import Path

from openpyxl import Workbook

OUT = Path(__file__).with_name("macros.xlsm")

# Module name → (procedural?, source). Sheet1 and Sheet2 are the document
# modules behind the Data and Summary sheets; Excel adds one per sheet, and
# they stay attribute-only unless someone writes event code in them.
DOC_ATTRS = """Attribute VB_Base = "0{00020820-0000-0000-C000-000000000046}"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = True
Attribute VB_TemplateDerived = False
Attribute VB_Customizable = True
"""

MODULES = {
    "ThisWorkbook": (False, 'Attribute VB_Name = "ThisWorkbook"\n' + DOC_ATTRS + """Option Explicit

Private Sub Workbook_Open()
    RollUp
End Sub
"""),
    "Sheet1": (False, 'Attribute VB_Name = "Sheet1"\n' + DOC_ATTRS),
    "Sheet2": (False, 'Attribute VB_Name = "Sheet2"\n' + DOC_ATTRS + "Option Explicit\n"),
    "Module1": (True, """Attribute VB_Name = "Module1"
Option Explicit

' Roll the month's revenue up into Summary.
Public Sub RollUp()
    Dim src As Worksheet
    Set src = ThisWorkbook.Worksheets("Data")
    With Worksheets("Summary")
        .Range("A1:B1").Value = Array("Region", "Revenue")
        .Range("B2:B5").ClearContents
    End With
    Totals src, Sheet2
    src.Range("A1:F1").Font.Bold = True
End Sub

Private Sub Totals(ws As Worksheet, dest As Worksheet)
    Dim i As Long
    For i = 2 To 9
        dest.Cells(2, 2).Value = dest.Cells(2, 2).Value + _
            ws.Cells(i, 5).Value
    Next i
    MsgBox "Total: " & ws.Range("E10").Value ' "Summary" here is just text
End Sub

Function Margin(price As Double, cost As Double) As Double
    Margin = (price - cost) / price
End Function
"""),
}


def build() -> bytes:
    wb = Workbook()
    wb.code_name = "ThisWorkbook"
    data = wb.active
    data.title = "Data"
    data.sheet_properties.codeName = "Sheet1"
    data.append(["Date", "Region", "Units", "Price", "Revenue", "Margin"])
    for i in range(8):
        r = i + 2
        data.append([f"2026-01-{i + 5:02}", ["North", "South", "East", "West"][i % 4], 10 + i, 4 + i, f"=C{r}*D{r}", f"=Margin(D{r},3)"])
    summary = wb.create_sheet("Summary")
    summary.sheet_properties.codeName = "Sheet2"
    summary["A1"], summary["B1"] = "Region", "Revenue"
    out = BytesIO()
    wb.save(out)
    return out.getvalue()


# ---------------------------------------------------------------- MS-OVBA


def compress(data: bytes) -> bytes:
    """MS-OVBA 2.4.1.3.1: a signature byte, then one chunk per 4096 bytes."""
    out = bytearray(b"\x01")
    for start in range(0, len(data), 4096):
        chunk = data[start:start + 4096]
        body = compress_chunk(chunk)
        if len(body) > 4096 and len(chunk) == 4096:
            out += struct.pack("<H", 0x3FFF) + chunk  # stored uncompressed
        else:
            out += struct.pack("<H", 0xB000 | (len(body) - 1)) + body
    return bytes(out)


def compress_chunk(chunk: bytes) -> bytes:
    body = bytearray()
    pos = 0
    while pos < len(chunk):
        flag_at = len(body)
        body.append(0)
        for bit in range(8):
            if pos >= len(chunk):
                break
            bit_count = max(4, (pos - 1).bit_length()) if pos > 1 else 4
            max_len = (0xFFFF >> bit_count) + 3
            best_len, best_off = 0, 0
            for cand in range(pos - 1, -1, -1):
                n = 0
                while n < max_len and pos + n < len(chunk) and chunk[cand + n] == chunk[pos + n]:
                    n += 1
                if n > best_len:
                    best_len, best_off = n, pos - cand
            if best_len >= 3:
                body[flag_at] |= 1 << bit
                body += struct.pack("<H", ((best_off - 1) << (16 - bit_count)) | (best_len - 3))
                pos += best_len
            else:
                body.append(chunk[pos])
                pos += 1
    return bytes(body)


def record(rid: int, payload: bytes) -> bytes:
    return struct.pack("<HI", rid, len(payload)) + payload


def dir_stream() -> bytes:
    """MS-OVBA 2.3.4.2: project information, no references, then the modules."""
    d = record(0x0001, struct.pack("<I", 1))  # SYSKIND: 32-bit Windows
    d += record(0x0002, struct.pack("<I", 0x409)) + record(0x0014, struct.pack("<I", 0x409))
    d += record(0x0003, struct.pack("<H", 1252))
    d += record(0x0004, b"VBAProject")
    d += record(0x0005, b"") + record(0x0040, b"")
    d += record(0x0006, b"") + record(0x003D, b"")
    d += record(0x0007, struct.pack("<I", 0)) + record(0x0008, struct.pack("<I", 0))
    d += struct.pack("<HIIH", 0x0009, 4, 1, 0)  # PROJECTVERSION
    d += record(0x000C, b"") + record(0x003C, b"")
    d += record(0x000F, struct.pack("<H", len(MODULES)))
    d += record(0x0013, struct.pack("<H", 0xFFFF))
    for name, (procedural, _) in MODULES.items():
        b = name.encode("cp1252")
        d += record(0x0019, b) + record(0x0047, name.encode("utf-16-le"))
        d += record(0x001A, b) + record(0x0032, name.encode("utf-16-le"))
        d += record(0x001C, b"") + record(0x0048, b"")
        d += record(0x0031, struct.pack("<I", 0))  # source starts the stream
        d += record(0x001E, struct.pack("<I", 0))
        d += record(0x002C, struct.pack("<H", 0xFFFF))
        d += struct.pack("<HI", 0x0021 if procedural else 0x0022, 0)
        d += struct.pack("<HI", 0x002B, 0)
    d += struct.pack("<HI", 0x0010, 0)
    return d


def project_stream() -> bytes:
    lines = ['ID="{00000000-0000-0000-0000-000000000000}"']
    for name, (procedural, _) in MODULES.items():
        lines.append(f"Module={name}" if procedural else f"Document={name}/&H00000000")
    lines += ['Name="VBAProject"', 'HelpContextID="0"', 'VersionCompatible32="393222000"', ""]
    lines += ["[Host Extender Info]", "&H00000001={3832D640-CF90-11CF-8E43-00A0C911005A};VBE;&H00000000", ""]
    return "\r\n".join(lines).encode("cp1252")


def vba_project() -> bytes:
    vba = {"dir": compress(dir_stream()), "_VBA_PROJECT": b"\xcc\x61\xff\xff\x00\x00\x00"}
    for name, (_, src) in MODULES.items():
        vba[name] = compress(src.replace("\n", "\r\n").encode("cp1252"))
    return cfb({"PROJECT": project_stream(), "VBA": vba})


# ---------------------------------------------------------------- MS-CFB

SECTOR, MINI, CUTOFF = 512, 64, 4096
FREE, END, FATSECT, NOSTREAM = 0xFFFFFFFF, 0xFFFFFFFE, 0xFFFFFFFD, 0xFFFFFFFF


def cfb(tree: dict) -> bytes:
    """A version-3 compound file. `tree` maps names to bytes (streams) or
    dicts (storages). Streams under 4096 bytes live in the mini stream."""
    entries = [{"name": "Root Entry", "type": 5, "kids": []}]

    def add(parent, items):
        for name, v in items.items():
            e = {"name": name, "type": 1 if isinstance(v, dict) else 2, "kids": [], "data": None if isinstance(v, dict) else v}
            entries.append(e)
            parent["kids"].append(len(entries) - 1)
            if isinstance(v, dict):
                add(e, v)

    add(entries[0], tree)

    mini = bytearray()
    minifat = []
    big = []  # (entry, data) stored in regular sectors
    for e in entries[1:]:
        if e["type"] != 2:
            continue
        if len(e["data"]) >= CUTOFF:
            big.append(e)
            continue
        n = max(1, -(-len(e["data"]) // MINI))
        e["start"] = len(minifat)
        minifat += [len(minifat) + i + 1 for i in range(n - 1)] + [END]
        mini += e["data"].ljust(n * MINI, b"\0")

    # Sector layout: FAT, directory, mini FAT, mini stream, big streams.
    dir_secs = -(-len(entries) // 4)
    minifat_secs = -(-len(minifat) * 4 // SECTOR) if minifat else 0
    mini_secs = -(-len(mini) // SECTOR)
    big_secs = [-(-len(e["data"]) // SECTOR) for e in big]
    rest = dir_secs + minifat_secs + mini_secs + sum(big_secs)
    fat_secs = 1
    while fat_secs * (SECTOR // 4) < fat_secs + rest:
        fat_secs += 1
    assert fat_secs <= 109, "needs DIFAT sectors"

    fat = [FATSECT] * fat_secs
    body = bytearray()

    def chain(data: bytes, n: int) -> int:
        start = len(fat)
        fat.extend([len(fat) + i + 1 for i in range(n - 1)] + [END])
        body.extend(data.ljust(n * SECTOR, b"\0"))
        return start

    dir_start = len(fat)
    fat.extend([len(fat) + i + 1 for i in range(dir_secs - 1)] + [END])
    dir_at = len(body)
    body.extend(b"\0" * dir_secs * SECTOR)
    minifat += [FREE] * (minifat_secs * SECTOR // 4 - len(minifat))
    minifat_start = chain(b"".join(struct.pack("<I", x) for x in minifat), minifat_secs) if minifat else END
    entries[0]["start"] = chain(bytes(mini), mini_secs) if mini else END
    entries[0]["data"] = bytes(mini)
    for e, n in zip(big, big_secs):
        e["start"] = chain(e["data"], n)
    fat += [FREE] * (fat_secs * SECTOR // 4 - len(fat))

    # Each storage's children form a binary search tree ordered by
    # (name length, upper-cased name); a balanced one, all black.
    def tree_of(ids):
        ids = sorted(ids, key=lambda i: (len(entries[i]["name"]), entries[i]["name"].upper()))

        def build(lo, hi):
            if lo >= hi:
                return NOSTREAM
            mid = (lo + hi) // 2
            entries[ids[mid]]["left"] = build(lo, mid)
            entries[ids[mid]]["right"] = build(mid + 1, hi)
            return ids[mid]

        return build(0, len(ids))

    for e in entries:
        e["child"] = tree_of(e["kids"]) if e["type"] != 2 else NOSTREAM

    raw = bytearray()
    for e in entries:
        name = e["name"].encode("utf-16-le") + b"\0\0"
        raw += name.ljust(64, b"\0") + struct.pack("<HBB", len(name), e["type"], 1)
        raw += struct.pack("<III", e.get("left", NOSTREAM), e.get("right", NOSTREAM), e["child"])
        raw += b"\0" * 16 + b"\0" * 4 + b"\0" * 16
        size = len(e["data"]) if e.get("data") is not None else 0
        raw += struct.pack("<IQ", e.get("start", 0) if e["type"] != 1 else 0, size)
    while len(raw) % SECTOR:
        raw += b"\0" * 64 + struct.pack("<HBBIII", 0, 0, 0, NOSTREAM, NOSTREAM, NOSTREAM) + b"\0" * 48
    body[dir_at:dir_at + len(raw)] = raw

    fat_bytes = b"".join(struct.pack("<I", x) for x in fat)
    header = b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1" + b"\0" * 16
    header += struct.pack("<HHHHH", 0x3E, 3, 0xFFFE, 9, 6) + b"\0" * 6
    header += struct.pack("<IIIIIIIII", 0, fat_secs, dir_start, 0, CUTOFF, minifat_start, minifat_secs, END, 0)
    difat = list(range(fat_secs)) + [FREE] * (109 - fat_secs)
    header += b"".join(struct.pack("<I", x) for x in difat)
    assert len(header) == SECTOR
    return header + fat_bytes + bytes(body)


# ---------------------------------------------------------------- package


def add_vba(xlsx: bytes) -> bytes:
    with zipfile.ZipFile(BytesIO(xlsx)) as z:
        files = {n: z.read(n) for n in z.namelist()}
    types = files["[Content_Types].xml"].decode()
    types = types.replace(
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
        "application/vnd.ms-excel.sheet.macroEnabled.main+xml",
    ).replace("</Types>", '<Default Extension="bin" ContentType="application/vnd.ms-office.vbaProject"/></Types>')
    rels = files["xl/_rels/workbook.xml.rels"].decode().replace(
        "</Relationships>",
        '<Relationship Id="rIdVba1" Type="http://schemas.microsoft.com/office/2006/relationships/vbaProject" Target="vbaProject.bin"/></Relationships>',
    )
    files.update({"[Content_Types].xml": types.encode(), "xl/_rels/workbook.xml.rels": rels.encode(), "xl/vbaProject.bin": vba_project()})
    out = BytesIO()
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
        for n in sorted(files, key=lambda n: (n != "[Content_Types].xml", n)):
            z.writestr(n, files[n])
    return out.getvalue()


if __name__ == "__main__":
    OUT.write_bytes(add_vba(build()))
    print(f"wrote {OUT}")
