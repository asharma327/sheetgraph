"""Build tests/fixtures/audit.xlsx: one workbook holding every item on the
coverage checklist (file-level metadata, tabs, cells, dependencies), so
sheetgraph's output can be checked item by item in tests/audit.rs.

openpyxl writes most of it. Images (floating and in a cell), a linked data
type, shapes, a form button, data connections, a Power Query and sparklines are
added afterwards as raw package parts.

    pip install openpyxl==3.1.5
    python tests/fixtures/make_audit.py
"""

import datetime as dt
import re
import base64
import struct
import zipfile
import zlib
from io import BytesIO
from pathlib import Path

from openpyxl import Workbook
from openpyxl.chart import BarChart, Reference
from openpyxl.comments import Comment
from openpyxl.formatting.rule import FormulaRule
from openpyxl.packaging.custom import StringProperty
from openpyxl.styles import Alignment, Border, Font, PatternFill, Protection, Side
from openpyxl.cell.rich_text import CellRichText, TextBlock
from openpyxl.cell.text import InlineFont
from openpyxl.workbook.defined_name import DefinedName
from openpyxl.workbook.protection import WorkbookProtection
from openpyxl.worksheet.datavalidation import DataValidation
from openpyxl.worksheet.formula import ArrayFormula
from openpyxl.worksheet.table import Table, TableStyleInfo

OUT = Path(__file__).with_name("audit.xlsx")
thin = Side(style="thin")


def build() -> bytes:
    wb = Workbook()
    wb.properties.creator = "Audit Author"
    wb.properties.title = "Audit Workbook"
    wb.properties.description = "Monthly close model"
    wb.properties.keywords = "finance; close"
    wb.properties.lastModifiedBy = "Last Editor"
    wb.properties.created = dt.datetime(2025, 3, 1, 9, 0)
    wb.custom_doc_props.append(StringProperty(name="Department", value="Finance"))
    wb.calculation.calcMode = "manual"
    wb.calculation.iterate = True
    wb.security = WorkbookProtection(lockStructure=True)

    # ---------------- Inputs: cell-level values and formatting
    inp = wb.active
    inp.title = "Inputs"
    inp["A1"] = "Item"
    inp["A1"].font = Font(bold=True, color="FFFFFF")
    inp["A1"].fill = PatternFill("solid", fgColor="1F4E79")
    inp["A1"].alignment = Alignment(horizontal="center")
    inp["A1"].border = Border(bottom=thin)
    inp["A2"] = "Widget"
    inp["B2"] = 12.5
    inp["B2"].number_format = "$#,##0.00"
    inp["C2"] = dt.date(2026, 1, 15)
    inp["C2"].number_format = "yyyy-mm-dd"
    inp["D2"] = True
    inp["E2"] = "#N/A"
    inp["E2"].data_type = "e"
    inp["A3"] = "Gadget"
    inp["A3"].font = Font(italic=True)
    inp["A4"] = "Gizmo"
    inp["A4"].font = Font(underline="single")
    inp["A5"] = "Retired"
    inp["A5"].font = Font(strike=True)
    inp["A6"] = "Big red"
    inp["A6"].font = Font(name="Courier New", size=14, color="FF0000")
    inp["B3"] = 0.25
    inp["B3"].number_format = "0%"
    inp["B4"] = 1234567
    inp["B4"].number_format = "#,##0"
    inp["B5"] = "00123"
    inp["B5"].quotePrefix = True
    inp["F2"] = CellRichText(["Total ", TextBlock(InlineFont(b=True, color="FF0000"), "due")])
    inp["G2"] = "see note"
    inp["G2"].comment = Comment("Check with finance", "Analyst")
    inp["H2"] = "site"
    inp["H2"].hyperlink = "https://example.com/policy"
    inp["H3"] = "go to calc"
    inp["H3"].hyperlink = "#Calc!A1"
    for r, v in enumerate(["Low", "Mid", "High"], start=2):
        inp.cell(r, 11, v)  # K2:K4
    dv1 = DataValidation(type="list", formula1='"Yes,No"', allow_blank=True)
    dv2 = DataValidation(type="list", formula1="=$K$2:$K$4")
    inp.add_data_validation(dv1)
    inp.add_data_validation(dv2)
    dv1.add("I2")
    dv2.add("I3")
    inp["I2"], inp["I3"] = "Yes", "Mid"
    inp["J2"] = 100
    inp["J2"].style = "Input"
    inp["L2"] = 5
    inp["L2"].protection = Protection(locked=False)
    inp["L3"] = "=1+1"
    inp["L3"].protection = Protection(hidden=True)
    inp["M2"] = "rotated"
    inp["M2"].alignment = Alignment(vertical="top", indent=2, textRotation=45)
    inp["A8"] = "Merged title"
    inp.merge_cells("A8:C8")
    inp["N2"] = "boxed"
    inp["N2"].border = Border(left=thin, right=thin, top=thin, bottom=thin)
    inp["N3"] = "underlined cell"
    inp["N3"].border = Border(bottom=Side(style="double"))
    inp["Z1"] = 1

    # ---------------- Report: table, chart, filter, freeze, tab colour, protection
    rep = wb.create_sheet("Report")
    rep.append(["Region", "Units", "Price"])
    for i, reg in enumerate(["North", "South", "East", "West", "Central"]):
        rep.append([reg, 10 + i, 2.5 + i])
    t = Table(displayName="Sales", ref="A1:C6")
    t.tableStyleInfo = TableStyleInfo(name="TableStyleMedium2", showRowStripes=True)
    rep.add_table(t)
    rep.freeze_panes = "B2"
    rep.sheet_properties.tabColor = "00B050"
    rep.protection.sheet = True
    ch = BarChart()
    ch.title = "Units by region"
    ch.add_data(Reference(rep, min_col=2, min_row=1, max_row=6), titles_from_data=True)
    ch.set_categories(Reference(rep, min_col=1, min_row=2, max_row=6))
    rep.add_chart(ch, "E2")
    rep.print_area = "A1:C6"
    rep["Z1"] = 2

    # ---------------- Calc: every kind of reference
    calc = wb.create_sheet("Calc")
    calc["A1"] = "=Inputs!B2*2"
    calc["A2"] = "=A1+1"
    calc["A3"] = "=TaxRate*100"
    calc["A4"] = "=SUM(Sales[Units])"
    calc["A5"] = "=SUM(Inputs:Report!Z1)"
    calc["A6"] = '=INDIRECT("Inputs!B2")'
    calc["A7"] = "=SUM(Inputs!B:B)"
    calc["A8"] = ArrayFormula("A8", "=SUM(Inputs!B2:B4*2)")
    calc["A9"] = "=_xlfn._xlws.SORT(Report!B2:B6)"
    for i, v in enumerate([11, 12, 13, 14], start=10):
        calc.cell(i, 1, v)  # spilled values A10:A13
    calc["B1"] = "=AddTax(10)"
    calc["B2"] = '=HYPERLINK("https://example.com","site")'
    calc["B3"] = "=LocalRate"
    calc["B4"] = "='[Other.xlsx]Data'!A1"
    calc["B5"] = "=1/0"
    calc["B6"] = '=VLOOKUP("a",Lookup!A1:B3,2,FALSE)'
    calc.conditional_formatting.add("A1:A8", FormulaRule(formula=["Inputs!$B$2>10"], fill=PatternFill("solid", bgColor="FFC7CE")))

    look = wb.create_sheet("Lookup")
    look.append(["a", 1])
    look.append(["b", 2])
    look.sheet_state = "hidden"
    sec = wb.create_sheet("Secret")
    sec["A1"] = "salary bands"
    sec.sheet_state = "veryHidden"

    # ---------------- Objects: layout, outline, gridlines (drawing parts added below)
    obj = wb.create_sheet("Objects")
    obj["A1"] = "Dashboard"
    for r in range(2, 10):
        obj.cell(r, 1, r)
    obj.column_dimensions["D"].width = 30
    obj.row_dimensions[2].height = 40
    for r in range(5, 9):
        obj.row_dimensions[r].outlineLevel = 1
    obj.sheet_view.showGridLines = False

    cs = wb.create_chartsheet("ChartSheet")
    ch2 = BarChart()
    ch2.title = "Calc values"
    ch2.add_data(Reference(calc, min_col=1, min_row=1, max_row=8))
    cs.add_chart(ch2)

    wb.defined_names["TaxRate"] = DefinedName("TaxRate", attr_text="Inputs!$B$3")
    wb.defined_names["AddTax"] = DefinedName("AddTax", attr_text="_xlfn.LAMBDA(_xlpm.x,_xlpm.x*(1+TaxRate))")
    calc.defined_names["LocalRate"] = DefinedName("LocalRate", attr_text="Calc!$A$1")

    out = BytesIO()
    wb.save(out)
    return out.getvalue()


def png() -> bytes:
    def chunk(t, d):
        return struct.pack(">I", len(d)) + t + d + struct.pack(">I", zlib.crc32(t + d) & 0xFFFFFFFF)

    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", 1, 1, 8, 2, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(b"\0\xff\0\0")) + chunk(b"IEND", b"")


R = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"
PKG = "http://schemas.openxmlformats.org/package/2006/relationships"

DRAWING = f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<xdr:wsDr xmlns:xdr="http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="{R}">
<xdr:twoCellAnchor><xdr:from><xdr:col>3</xdr:col><xdr:colOff>0</xdr:colOff><xdr:row>1</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:from><xdr:to><xdr:col>5</xdr:col><xdr:colOff>0</xdr:colOff><xdr:row>6</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:to>
<xdr:pic><xdr:nvPicPr><xdr:cNvPr id="2" name="Logo" descr="Company logo"/><xdr:cNvPicPr/></xdr:nvPicPr><xdr:blipFill><a:blip r:embed="rIdImg"/><a:stretch><a:fillRect/></a:stretch></xdr:blipFill><xdr:spPr><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></xdr:spPr></xdr:pic><xdr:clientData/></xdr:twoCellAnchor>
<xdr:twoCellAnchor><xdr:from><xdr:col>6</xdr:col><xdr:colOff>0</xdr:colOff><xdr:row>1</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:from><xdr:to><xdr:col>9</xdr:col><xdr:colOff>0</xdr:colOff><xdr:row>4</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:to>
<xdr:sp macro="" textlink=""><xdr:nvSpPr><xdr:cNvPr id="3" name="TextBox 1"/><xdr:cNvSpPr txBox="1"/></xdr:nvSpPr><xdr:spPr><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></xdr:spPr><xdr:txBody><a:bodyPr/><a:p><a:r><a:t>Rates update on the 1st of each month</a:t></a:r></a:p></xdr:txBody></xdr:sp><xdr:clientData/></xdr:twoCellAnchor>
</xdr:wsDr>"""

VML = """<xml xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:x="urn:schemas-microsoft-com:office:excel">
<v:shape id="_x0000_s1025" type="#_x0000_t201" style="position:absolute;margin-left:300pt;margin-top:120pt;width:90pt;height:24pt">
<v:textbox><div><font>Run roll-up</font></div></v:textbox>
<x:ClientData ObjectType="Button"><x:Anchor>6, 0, 8, 0, 7, 0, 9, 0</x:Anchor><x:FmlaMacro>[0]!RollUp</x:FmlaMacro><x:TextHAlign>Center</x:TextHAlign></x:ClientData>
</v:shape></xml>"""

SPARKLINES = """<extLst><ext uri="{05C60535-1F16-4fd2-B633-F4F36F0B64E0}" xmlns:x14="http://schemas.microsoft.com/office/spreadsheetml/2009/9/main"><x14:sparklineGroups xmlns:xm="http://schemas.microsoft.com/office/excel/2006/main"><x14:sparklineGroup type="line"><x14:sparklines><x14:sparkline><xm:f>Objects!A2:A9</xm:f><xm:sqref>B2</xm:sqref></x14:sparkline></x14:sparklines></x14:sparklineGroup></x14:sparklineGroups></ext></extLst>"""

CONNECTIONS = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<connections xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">\
<connection id="1" name="Prices DB" type="1" refreshedVersion="6" refreshOnLoad="1"><dbPr connection="DRIVER={SQL Server};SERVER=sql01;DATABASE=Prices;UID=report;PWD=hunter2" command="SELECT * FROM dbo.prices" commandType="2"/></connection>\
<connection id="2" name="Query - Prices" description="Connection to the 'Prices' query in the workbook." type="5" refreshedVersion="8" background="1" saveData="1"><dbPr connection="Provider=Microsoft.Mashup.OleDb.1;Data Source=$Workbook$;Location=Prices;Extended Properties=&quot;&quot;" command="SELECT * FROM [Prices]"/></connection>\
</connections>"""

# Pictures in cells and linked data types: cell vm → metadata.xml → rich value → structure.
RD = "http://schemas.microsoft.com/office/spreadsheetml/2017/richdata"
METADATA = f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<metadata xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:xlrd="{RD}">\
<metadataTypes count="1"><metadataType name="XLRICHVALUE" minSupportedVersion="120000" copy="1" pasteAll="1" pasteValues="1" merge="1" splitFirst="1" rowColShift="1" clearFormats="1" clearComments="1" assign="1" coerce="1"/></metadataTypes>\
<futureMetadata name="XLRICHVALUE" count="2"><bk><extLst><ext uri="{{3e2802c4-a4d2-4d8b-9148-e3be6c30e623}}"><xlrd:rvb i="0"/></ext></extLst></bk><bk><extLst><ext uri="{{3e2802c4-a4d2-4d8b-9148-e3be6c30e623}}"><xlrd:rvb i="1"/></ext></extLst></bk></futureMetadata>\
<valueMetadata count="2"><bk><rc t="1" v="0"/></bk><bk><rc t="1" v="1"/></bk></valueMetadata></metadata>"""
RICH_STRUCTURES = f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<rvStructures xmlns="{RD}" count="2">\
<s t="_localImage"><k n="_rvRel:LocalImageIdentifier" t="i"/><k n="CalcOrigin" t="i"/><k n="Text" t="s"/></s>\
<s t="_linkedEntity"><k n="%EntityServiceId" t="i"/><k n="_DisplayString" t="s"/><k n="%EntityId" t="s"/><k n="_Icon" t="s"/><k n="Name" t="s"/><k n="Price"/><k n="Exchange" t="s"/></s>\
</rvStructures>"""
RICH_VALUES = f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<rvData xmlns="{RD}" count="2">\
<rv s="0"><v>0</v><v>5</v><v>Product photo</v></rv>\
<rv s="1"><v>268435456</v><v>Microsoft Corp</v><v>XNAS:MSFT</v><v>Stocks</v><v>Microsoft Corporation</v><v>412.5</v><v>NASDAQ</v></rv>\
</rvData>"""
RICH_RELS = f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<richValueRels xmlns="http://schemas.microsoft.com/office/spreadsheetml/2022/richvaluerel" xmlns:r="{R}"><rel r:id="rId1"/></richValueRels>"""

SECTION1 = """section Section1;

shared Prices = let
    Source = Sql.Database("sql01", "Prices"),
    Rows = Source{[Schema="dbo",Item="prices"]}[Data]
in
    Rows;

shared #"Top Prices" = let
    Source = Prices,
    Sorted = Table.Sort(Source, {{"price", Order.Descending}})
in
    Table.FirstN(Sorted, 10);
"""


def data_mashup() -> bytes:
    """customXml item holding Power Query: base64 of [version][length][zip][…], in UTF-16."""
    inner = BytesIO()
    with zipfile.ZipFile(inner, "w", zipfile.ZIP_DEFLATED) as z:
        z.writestr("[Content_Types].xml", '<?xml version="1.0" encoding="utf-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="xml" ContentType="text/xml"/><Default Extension="m" ContentType="application/x-ms-m"/></Types>')
        z.writestr("Config/Package.xml", '<?xml version="1.0" encoding="utf-8"?><Package xmlns="http://schemas.microsoft.com/DataMashup"><Version>2.0</Version><MinVersion>2.0</MinVersion><Culture>en-US</Culture></Package>')
        z.writestr("Formulas/Section1.m", SECTION1)
    parts = inner.getvalue()
    blob = struct.pack("<II", 0, len(parts)) + parts + struct.pack("<III", 0, 0, 0)
    xml = '<?xml version="1.0" encoding="utf-16"?><DataMashup xmlns="http://schemas.microsoft.com/DataMashup">' + base64.b64encode(blob).decode() + "</DataMashup>"
    return b"\xff\xfe" + xml.encode("utf-16-le")


def add_parts(xlsx: bytes) -> bytes:
    with zipfile.ZipFile(BytesIO(xlsx)) as z:
        files = {n: z.read(n) for n in z.namelist()}
    wbxml = files["xl/workbook.xml"].decode()
    rels = files["xl/_rels/workbook.xml.rels"].decode()
    rid = re.search(r'name="Objects"[^/]*?id="(rId\d+)"', wbxml).group(1)
    target = re.search(rf'Id="{rid}"[^>]*Target="([^"]+)"|Target="([^"]+)"[^>]*Id="{rid}"', rels)
    target = (target.group(1) or target.group(2)).lstrip("/")
    sheet = target if target.startswith("xl/") else "xl/" + target
    name = sheet.rsplit("/", 1)[1]
    sheet_rels = f"xl/worksheets/_rels/{name}.rels"

    xml = files[sheet].decode()
    xml = xml.replace("</worksheet>", f'<drawing r:id="rIdObjD"/><legacyDrawing r:id="rIdObjV"/>{SPARKLINES}</worksheet>')
    if 'xmlns:r=' not in xml.split(">", 1)[0]:
        xml = xml.replace("<worksheet ", f'<worksheet xmlns:r="{R}" ', 1)
    files[sheet] = xml.encode()
    files[sheet_rels] = (
        f'<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n<Relationships xmlns="{PKG}">'
        f'<Relationship Id="rIdObjD" Type="{R}/drawing" Target="../drawings/drawingObj.xml"/>'
        f'<Relationship Id="rIdObjV" Type="{R}/vmlDrawing" Target="../drawings/vmlDrawingObj.vml"/></Relationships>'
    ).encode()
    files["xl/drawings/drawingObj.xml"] = DRAWING.encode()
    files["xl/drawings/_rels/drawingObj.xml.rels"] = (
        f'<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n<Relationships xmlns="{PKG}">'
        f'<Relationship Id="rIdImg" Type="{R}/image" Target="../media/logo.png"/></Relationships>'
    ).encode()
    files["xl/drawings/vmlDrawingObj.vml"] = VML.encode()
    files["xl/media/logo.png"] = png()
    files["xl/connections.xml"] = CONNECTIONS.encode()
    files["xl/metadata.xml"] = METADATA.encode()
    files["xl/richData/rdrichvaluestructure.xml"] = RICH_STRUCTURES.encode()
    files["xl/richData/rdrichvalue.xml"] = RICH_VALUES.encode()
    files["xl/richData/richValueRel.xml"] = RICH_RELS.encode()
    files["xl/richData/_rels/richValueRel.xml.rels"] = (
        f'<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n<Relationships xmlns="{PKG}">'
        f'<Relationship Id="rId1" Type="{R}/image" Target="../media/photo.png"/></Relationships>'
    ).encode()
    files["xl/media/photo.png"] = png()
    files["customXml/item1.xml"] = data_mashup()
    files["xl/_rels/workbook.xml.rels"] = rels.replace(
        "</Relationships>",
        f'<Relationship Id="rIdConn1" Type="{R}/connections" Target="connections.xml"/>'
        f'<Relationship Id="rIdMeta1" Type="{R}/sheetMetadata" Target="metadata.xml"/>'
        '<Relationship Id="rIdRv1" Type="http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue" Target="richData/rdrichvalue.xml"/>'
        '<Relationship Id="rIdRv2" Type="http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure" Target="richData/rdrichvaluestructure.xml"/>'
        '<Relationship Id="rIdRv3" Type="http://schemas.microsoft.com/office/2022/10/relationships/richValueRel" Target="richData/richValueRel.xml"/>'
        f'<Relationship Id="rIdCx1" Type="{R}/customXml" Target="../customXml/item1.xml"/></Relationships>',
    ).encode()

    # Inputs!P2 holds a picture, Inputs!Q2 a stock; both store #VALUE! as their value.
    inputs = "xl/worksheets/sheet1.xml"
    xml = files[inputs].decode()
    row2 = re.search(r'<row r="2"[^>]*>.*?</row>', xml, re.S).group(0)
    cells = '<c r="P2" t="e" vm="1"><v>#VALUE!</v></c><c r="Q2" t="e" vm="2"><v>#VALUE!</v></c>'
    xml = xml.replace(row2, row2.replace("</row>", cells + "</row>"))
    xml = re.sub(r'<dimension ref="[^"]*"/>', '<dimension ref="A1:Z8"/>', xml)
    files[inputs] = xml.encode()
    types = files["[Content_Types].xml"].decode()
    types = types.replace(
        "</Types>",
        '<Default Extension="png" ContentType="image/png"/>'
        '<Default Extension="vml" ContentType="application/vnd.openxmlformats-officedocument.vmlDrawing"/>'
        '<Override PartName="/xl/drawings/drawingObj.xml" ContentType="application/vnd.openxmlformats-officedocument.drawing+xml"/>'
        '<Override PartName="/xl/connections.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"/>'
        '<Override PartName="/xl/metadata.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"/>'
        '<Override PartName="/xl/richData/rdrichvalue.xml" ContentType="application/vnd.ms-excel.rdrichvalue+xml"/>'
        '<Override PartName="/xl/richData/rdrichvaluestructure.xml" ContentType="application/vnd.ms-excel.rdrichvaluestructure+xml"/>'
        '<Override PartName="/xl/richData/richValueRel.xml" ContentType="application/vnd.ms-excel.richvaluerel+xml"/>'
        "</Types>",
    )
    files["[Content_Types].xml"] = types.encode()
    out = BytesIO()
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
        for n in sorted(files, key=lambda n: (n != "[Content_Types].xml", n)):
            z.writestr(n, files[n])
    return out.getvalue()


if __name__ == "__main__":
    OUT.write_bytes(add_parts(build()))
    print(f"wrote {OUT}")
