# ใบตรวจการใช้วัตถุดิบ (CSD)
gsheet 1znYsQM9ueZVxVbdTdrsnjbZuMSWhRu-z3oQZbwQjW5M · en_US · Asia/Bangkok · modified 2026-10-23 · owner accounting@hunterpoke.com
23 sheets · 2.8M non-empty cells · 1.5M formulas in 1671 copy-groups · 50k error cells
Notation: ƒN = formula copy-group N of a sheet; a group's formula is shown at its top-left cell and is copied (relative refs shift) over its range. ↓ filled down, → across, ▦ both.

## Sheets
1. Template — C1:C3 · 3 cells
2. Sales — A1:L2202 · 26k cells · 1 ƒ in 1 groups
3. Monthly Summary (Usage) — C1:BI246 · 8331 cells · 8133 ƒ in 233 groups · reads Sales, Item Sales (Cal), Item Sales (POS), Fish record, Salmon, Tuna, Chicken, ChickenBB, Avocado, Mango, MahachanokMango, Pomelo, Tamago, Edamame, Ikura, Others
4. Monthly Summary (item sales) — B1:CT358 · 14k cells · 13k ƒ in 239 groups · reads Item Sales (POS)
5. Item Sales (Cal) — A1:BPD1148 · 1.0M cells · 1.0M ƒ in 306 groups · reads Item Sales (POS)
6. Item Sales (POS) — A1:BMW874 · 997k cells · 11k ƒ in 11 groups
7. Item Sales (Usage) — A1:M2000 · 24k cells · 22k ƒ in 13 groups · reads Item Sales (Cal)
8. Monthly Inventory Summary — A1:AE35 · 941 cells · 835 ƒ in 429 groups · reads Salmon, Tuna, Avocado, Mango, MahachanokMango, Pomelo, Tamago, Edamame, Eel, Ikura, Others
9. Inventory Summary — A1:I34 · 118 cells · 79 ƒ in 79 groups · reads Fish record, Salmon, Tuna, Avocado, Mango, MahachanokMango, Pomelo, Tamago, Edamame, Eel, Ikura, Others
10. Fish record — A1:W1869 · 27k cells · 11k ƒ in 9 groups
11. Salmon — A1:V3001 · 52k cells · 42k ƒ in 23 groups · reads Sales, Item Sales (Usage), Fish record
12. Tuna — A1:V2000 · 37k cells · 28k ƒ in 22 groups · reads Sales, Item Sales (Usage), Fish record
13. Chicken — A1:V1570 · 24k cells · 16k ƒ in 17 groups · reads Sales, Item Sales (Usage)
14. ChickenBB — A1:V2500 · 32k cells · 25k ƒ in 18 groups · reads Sales, Item Sales (Usage)
15. Avocado — A1:N2000 · 23k cells · 14k ƒ in 10 groups · reads Sales, Item Sales (Usage)
16. Mango — A1:N2000 · 22k cells · 14k ƒ in 11 groups · reads Sales, Item Sales (Usage)
17. MahachanokMango — A1:N2000 · 20k cells · 14k ƒ in 10 groups · reads Sales, Item Sales (Usage)
18. Pomelo — A1:V2000 · 32k cells · 20k ƒ in 17 groups · reads Sales, Item Sales (Usage)
19. Tamago — A1:V2000 · 31k cells · 20k ƒ in 17 groups · reads Sales, Item Sales (Usage)
20. Edamame — A1:V2000 · 31k cells · 20k ƒ in 17 groups · reads Sales, Item Sales (Usage)
21. Eel — A1:R2000 · 26k cells · 14k ƒ in 13 groups · reads Sales
22. Ikura — A1:V2000 · 30k cells · 20k ƒ in 17 groups · reads Sales, Item Sales (Usage)
23. Others — A1:HW2681 · 311k cells · 153k ƒ in 159 groups · reads Sales

## Data flow (source → readers, by formula refs)
Sales → Monthly Summary (Usage) (6g), Salmon (1g), Tuna (2g), Chicken (1g), ChickenBB (2g), Avocado (2g), Mango (2g), MahachanokMango (2g), Pomelo (2g), Tamago (2g), Edamame (2g), Eel (2g), Ikura (2g), Others (38g)
Item Sales (Cal) → Monthly Summary (Usage) (6g), Item Sales (Usage) (10g)
Item Sales (POS) → Monthly Summary (Usage) (1g), Monthly Summary (item sales) (96g), Item Sales (Cal) (261g)
Item Sales (Usage) → Salmon (2g), Tuna (2g), Chicken (2g), ChickenBB (2g), Avocado (2g), Mango (3g), MahachanokMango (2g), Pomelo (2g), Tamago (2g), Edamame (2g), Ikura (2g)
Fish record → Monthly Summary (Usage) (16g), Inventory Summary (6g), Salmon (7g), Tuna (7g)
Salmon → Monthly Summary (Usage) (4g), Monthly Inventory Summary (14g), Inventory Summary (3g)
Tuna → Monthly Summary (Usage) (2g), Monthly Inventory Summary (14g), Inventory Summary (2g)
Chicken → Monthly Summary (Usage) (12g)
ChickenBB → Monthly Summary (Usage) (1g)
Avocado → Monthly Summary (Usage) (8g), Monthly Inventory Summary (7g), Inventory Summary (3g)
Mango → Monthly Summary (Usage) (8g), Monthly Inventory Summary (7g), Inventory Summary (3g)
MahachanokMango → Monthly Summary (Usage) (8g), Monthly Inventory Summary (7g), Inventory Summary (1g)
Pomelo → Monthly Summary (Usage) (14g), Monthly Inventory Summary (14g), Inventory Summary (3g)
Tamago → Monthly Summary (Usage) (14g), Monthly Inventory Summary (14g), Inventory Summary (5g)
Edamame → Monthly Summary (Usage) (14g), Monthly Inventory Summary (14g), Inventory Summary (5g)
Eel → Monthly Inventory Summary (14g), Inventory Summary (2g)
Ikura → Monthly Summary (Usage) (14g), Monthly Inventory Summary (14g), Inventory Summary (2g)
Others → Monthly Summary (Usage) (36g), Monthly Inventory Summary (259g), Inventory Summary (42g)

## External links
IMPORTRANGE 19pXjj0ZWWhr3kcWDRrT3w3Dxd88elcT1x1rKM1I1W6M (1): "Sales(ใบตรวจร้าน)!A1:L3000"→Sales!A1
IMPORTRANGE 1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g (40): "Fish record summary!D3:g2000"→'Fish record'!D3, "Fish record summary!j3:m2000"→'Fish record'!J3, "1/1!c3:h2000"→Salmon!E4, "1/1!k3:k2000"→Salmon!V4, "1/2!c3:h11"→Tuna!E3, "1/2!c3:c2000"→Tuna!V3, "1/1!s12:x2000"→Tuna!E12, "9/2!K460:P2000"→Chicken!E3, "10/3!C3:H2000"→ChickenBB!E3, "2/1!c3:h2000"→Avocado!E3, "1/2!k3:p2000"→Mango!E3, "1/2!s3:x2000"→MahachanokMango!E3 +28
IMPORTRANGE 1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o (88): "5/3!E460:E2000"→Chicken!K3, "5/3!J460:J2000"→Chicken!L3, "5/3!G460:G2000"→Chicken!M3, "5/3!H460:H2000"→Chicken!N3, "5/3!C460:C2000"→Chicken!O3, "5/3!N460:N2000"→ChickenBB!K3, "5/3!S460:S2000"→ChickenBB!L3, "5/3!P460:P2000"→ChickenBB!M3, "5/3!Q460:Q2000"→ChickenBB!N3, "5/3!L460:L2000"→ChickenBB!O3, "3/1!n3:n2000"→Pomelo!K3, "3/1!s3:s2000"→Pomelo!L3 +76

## Functions (cells using)
AND 153k · IF 90k · IFERROR 66k · OR 62k · YEAR 35k · MONTH 35k · SUMIFS 29k · HLOOKUP 18k · SUM 13k · SUMPRODUCT 1250 · INDEX 455 · MATCH 455 · IMPORTRANGE 129 · DAY 51 · EOMONTH 33 · TODAY 2
volatile: TODAY 2

## Extraction notes
- Apps Script (bound scripts, macros, add-ons, triggers) is not exposed by the Sheets API; pass --apps-script <scriptId> with user credentials to include a project

## 1. Template · C1:C3 · 3 cells
▸ C1:C3 (3 cells)
  1: C=importrange
  2: C=Manual cal
  3: C=Formula

## 2. Sales · A1:L2202 · 26k cells · frozen 0r3c
read by: Monthly Summary (Usage), Salmon, Tuna, Chicken, ChickenBB, Avocado, Mango, MahachanokMango, Pomelo, Tamago, Edamame, Eel, Ikura, Others
▸ A1:L2202 table 2202×12 (26413 cells), header row 3
  A "Date": date Tue 01-03-2022  … Tue 07-03-2028  (2199 filled)
  B "Month": num 1 … 12 (2199 filled)
  C "Year": num 2022 … 2028 (2199 filled)
  D "[17  / TCM] Total": num 0.00 … 50,292.00; 432 errors (2199 filled)
  E "[6 ] instore": num 0.00 … 23,785.00; 432 errors (2199 filled)
  F "[11 ] delivery": num 0.00 … 37,975.00; 432 errors (2199 filled)
  G "[33  / EMQ] Total": num 0.00 … 34,600.00; 432 errors (2199 filled)
  H "[20 ] instore": num 0.00 … 28,345.00; 432 errors (2199 filled)
  I "[26 ] delivery": num 0.00 … 23,435.00; 432 errors (2199 filled)
  J "[47  / CSD] Total": num 0.00 … 41,365.50; 432 errors (2199 filled)
  K "[36 ] instore": num 0.00 … 18,820.00; 432 errors (2199 filled)
  L "[41 ] delivery": num 0.00 … 35,300.00; 432 errors (2199 filled)
  sample (A…L):
  4| Tue 01-03-2022  | 3 | 2022 | 30,682.00 | 6,780.00 | 23,902.00 | 14,872.00 | 9,690.00 | 5,182.00 | 24,548.00 | 5,800.00 | 18,748.00
  5| Wed 02-03-2022  | 3 | 2022 | 34,837.00 | 8,980.00 | 25,857.00 | 15,002.00 | 8,550.00 | 6,452.00 | 30,578.00 | 6,786.00 | 23,792.00
  6| Thu 03-03-2022  | 3 | 2022 | 28,346.00 | 10,011.00 | 18,335.00 | 14,475.00 | 10,330.00 | 4,145.00 | 22,071.00 | 5,776.00 | 16,295.00
  2202| Tue 07-03-2028  | 3 | 2028 | #N/A | #N/A | #N/A | #N/A | #N/A | #N/A | #N/A | #N/A | #N/A
ƒ formulas (1 cells, 1 groups, 1 families)
ƒ1 A1 =importrange("https://docs.google.com/spreadsheets/d/19pXjj0ZWWhr3kcWDRrT3w3Dxd88elcT1x1rKM1I1W6M/edit#gid=35816577","Sales(ใบตรวจร้าน)!A1:L3000") [external] [array/spills]
◆ formatting
bg#FCE5CD (1 cells): A1

## 3. Monthly Summary (Usage) · C1:BI246 · 8331 cells · frozen 3r4c
reads: Sales (6g), Item Sales (Cal) (6g), Item Sales (POS) (1g), Fish record (16g), Salmon (4g), Tuna (2g), Chicken (12g), ChickenBB (1g), Avocado (8g), Mango (8g), MahachanokMango (8g), Pomelo (14g), Tamago (14g), Edamame (14g), Ikura (14g), Others (36g)
▸ C1:BI19 row-oriented 19×59 (1070 cells), labels in column C
  1 "Year": ƒ1; num 2022 … 2026 (57 filled)
  2 "Month": ƒ2; num 1 … 12 (57 filled)
  3 "Date": ƒ3; date 1/31/2022 … 9/30/2026 (57 filled)
  4 "Average daily sale": ƒ4; num 18,504.06 … 30,708.85 fmt #,##0.00 (51 filled)
  5 "CSD ยอดขายเฉพาะPOKE": ƒ5,ƒ6; num 555,343.95 … 951,974.25 fmt #,##0.00 (55 filled)
  6 "instore": ƒ7,ƒ8; num 153,551.00 … 332,483.00 fmt #,##0.00 (55 filled)
  7 "Delivery": ƒ9,ƒ10; num 288,735.00 … 696,379.00 fmt #,##0.00 (55 filled)
  8 "แซลมอน (กรัม/1000บาท)": ƒ130; num 64.53 ก. … 159.56 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (57 filled)
  9 "แซลมอนทั้งหมด": ƒ11,ƒ12; num 49,970.00 … 128,830.00 fmt #,##0.00 (57 filled)
  10 "Yield": ƒ141; num 83.74% … 92.42% fmt 0.00% (55 filled)
  11 "waste": ƒ13,ƒ14; num 4271 … 11769 (55 filled)
  12 "closed": ƒ15,ƒ16; num 664 … 9585 (55 filled)
  13 "expired": ƒ148,ƒ149; num 0 … 1946 (55 filled)
  14 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ150; num 5,518.00 … 20,702.00 fmt #,##0.00 (55 filled)
  15 "ปริมาณทิ้งทั้งหมด (%)": ƒ157; num 8.86% … 28.94% fmt 0.00% (55 filled)
  16 "ใบนับของสด-ใบจดปลา (กรัม)": ƒ17,ƒ18; num 2,380.00 … 5,984.00 fmt #,##0.00 (55 filled)
  17 "ใบนับของสด-ใบจดปลา (%)": ƒ158; num 3.44% … 8.64% fmt 0.00% (55 filled)
  18 "ใบจดปลา-Item Sales(กรัม)": ƒ19,ƒ20; num 1,043.00 … 4,257.00 fmt #,##0.00 (55 filled)
  19 "ใบจดปลา-Item Sales (%)": ƒ174; num 2.22% … 6.11% fmt 0.00% (55 filled)
▸ G21:BI27 table 7×55 (385 cells), no header row
  G…BI (55 alike): ƒ21,ƒ181,ƒ185 +4; num 131.99 ก. … 85,590.00 fmt #,##0.00 (7 filled)
  sample (G…V of 55):
  21| 85,590.00 | 87,660.00 | 57,750.00 | 55,455.00 | 47,265.00 | 66,210.00 | 73,290.00 | 73,560.00 | 73,404.00 | 68,672.00 | 89,019.00 | 48,151.00 | 60,152.00 | 54,107.00 | 48,549.00 | 51,480.00
  22| 178.75 ก. | 180.38 ก. | 133.61 ก. | 120.11 ก. | 133.70 ก. | 155.24 ก. | 176.36 ก. | 169.18 ก. | 170.48 ก. | 165.90 ก. | 318.74 ก. | 287.35 ก. | 229.32 ก. | 280.55 ก. | 255.33 ก. | 211.68 ก.
  23| 31,619.80 | 33,117.04 | 22,861.45 | 21,729.03 | 20,907.58 | 30,068.20 | 33,673.17 | 36,156.98 | 37,635.33 | 38,473.76 | 89,019.00 | 48,151.00 | 60,152.00 | 54,107.00 | 48,549.00 | 51,480.00
  27| 13053.99 | 12872.29 | 9942.78 | 7679.95 | 10384.04 | 6506.72 | 8241.21 | 8728.90 | 7928.13 | 7171.82 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00
▸ C29:BI40 row-oriented 12×59 (676 cells), labels in column C
  29 "ทูน่า (กรัม/1000บาท)": ƒ131; num 30.67 ก. … 56.53 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (57 filled)
  30 "ทูน่าทั้งหมด": ƒ24,ƒ25; num 20,367.00 … 51,003.00 fmt #,##0.00 (57 filled)
  31 "Yield": ƒ142; num 98.70% … 100.00% fmt 0.00% (55 filled)
  32 "waste": ƒ26,ƒ27; num 0 … 398 (55 filled)
  33 "closed": ƒ28,ƒ29; num 19 … 2977 (55 filled)
  34 "expired": ƒ30,ƒ31; num 0 (55 filled)
  35 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ151; num 36.00 … 2,992.00 fmt #,##0.00 (55 filled)
  36 "ปริมาณทิ้งทั้งหมด (%)": ƒ159; num 0.11% … 11.84% fmt 0.00% (55 filled)
  37 "ใบนับของสด-ใบจดปลา (กรัม)": ƒ32; num 0.00 … 2,555.00 fmt #,##0.00 (55 filled)
  38 "ใบนับของสด-ใบจดปลา (%)": ƒ160; num 0.00% … 8.71% fmt 0.00% (55 filled)
  39 "ใบจดปลา-Item Sales(กรัม)": ƒ33; num 0.00 … 1,960.00 fmt #,##0.00 (55 filled)
  40 "ใบจดปลา-Item Sales (%)": ƒ175; num 0.00% … 4.80% fmt 0.00% (55 filled)
▸ G42:BI48 table 7×55 (385 cells), no header row
  G…BI (55 alike): ƒ34,ƒ183,ƒ186 +4; num 24.38 ก. … 23,850.00 fmt #,##0.00 (7 filled)
  sample (G…V of 55):
  42| 23,850.00 | 23,895.00 | 31,770.00 | 32,565.00 | 26,445.00 | 21,300.00 | 20,940.00 | 23,130.00 | 21,414.00 | 17,537.00 | 19,134.00 | 26,536.00 | 35,867.00 | 31,802.00 | 31,854.00 | 29,640.00
  43| 61.63 ก. | 65.23 ก. | 75.37 ก. | 80.70 ก. | 69.89 ก. | 59.65 ก. | 53.49 ก. | 56.66 ก. | 50.61 ก. | 43.54 ก. | 63.74 ก. | 126.42 ก. | 117.46 ก. | 135.43 ก. | 144.31 ก. | 107.07 ก.
  44| 12,458.67 | 13,153.93 | 15,488.34 | 16,606.24 | 13,931.90 | 11,972.19 | 11,127.96 | 13,745.79 | 12,344.67 | 11,094.81 | 19,134.00 | 26,536.00 | 35,867.00 | 31,802.00 | 31,854.00 | 29,640.00
  48| 987.25 | 1344.94 | 1138.23 | 1266.80 | 1170.63 | 1294.51 | 1066.02 | 812.65 | 957.59 | 813.68 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00
▸ C50:BI56 row-oriented 7×59 (396 cells), labels in column C
  50 "มะม่วง (กรัม/1000บาท)": ƒ132; num 0.00 ก. … 141.98 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (57 filled)
  51 "มะม่วงทั้งหมด": ƒ37,ƒ38; num 0.00 … 104,788.00 fmt #,##0.00 (57 filled)
  52 "Yield": ƒ191; num 20.77% … 292.90% fmt 0.00%; 2 errors (55 filled)
  53 "waste ใบของสด": ƒ39,ƒ40; num 0 … 18109 (55 filled)
  54 "expired": ƒ41,ƒ42; num 0 … 3568 (55 filled)
  55 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ43,ƒ44; num -18,942.00 … 70,490.00 fmt #,##0.00 (55 filled)
  56 "ปริมาณทิ้งทั้งหมด (%)": ƒ161; num -192.90% … 74.73% fmt 0.00%; 2 errors (55 filled)
▸ C59:BI124 row-oriented 66×59 (3704 cells), labels in column C
  59 "อโวคาโด (กรัม/1000บาท)": ƒ133; num 68.17 ก. … 114.04 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (57 filled)
  60 "อโวคาโดทั้งหมด": ƒ45,ƒ46; num 46,465.00 … 89,653.00 fmt #,##0.00 (57 filled)
  61 "Yield": ƒ192; num 25.00% … 46.67% fmt 0.00% (55 filled)
  62 "waste ใบของสด": ƒ47,ƒ48; num 829 … 33195 (55 filled)
  63 "expired": ƒ49,ƒ50; num 528 … 5680 (55 filled)
  64 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ51,ƒ52; num 25,069.00 … 61,705.00 fmt #,##0.00 (55 filled)
  65 "ปริมาณทิ้งทั้งหมด (%)": ƒ162; num 48.83% … 68.83% fmt 0.00% (55 filled)
  66 "มะม่วงมหาชนก (กรัม/1000บาท)": ƒ134,ƒ135; num 0.00 ก. … 109.53 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (55 filled)
  67 "มะม่วงมหาชนกทั้งหมด": ƒ53,ƒ54; num 0 … 74,468.00 fmt #,##0.00 (55 filled)
  68 "Yield": ƒ193; num 23.39% … 880.23% fmt 0.00%; 42 errors (55 filled)
  69 "waste ใบของสด": ƒ55,ƒ56; num 0 … 7684 (55 filled)
  70 "expired": ƒ57,ƒ58; num 0 … 2674 (55 filled)
  71 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ59,ƒ60; num -28,116.00 … 49,771.00 fmt #,##0.00 (55 filled)
  72 "ปริมาณทิ้งทั้งหมด (%)": ƒ163; num -789.36% … 72.43% fmt 0.00%; 42 errors (55 filled)
  73 "ส้มโอ (กรัม/1000บาท)": ƒ136; num 13.69 ก. … 36.74 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (57 filled)
  74 "ส้มโอ ทั้งหมด": ƒ61,ƒ62; num 10,114.00 … 25,460.00 fmt #,##0.00 (57 filled)
  75 "Yield": ƒ143; num 94.36% … 102.18% fmt 0.00% (55 filled)
  76 "waste ใบของสด": ƒ63,ƒ64; num -306 … 942 (55 filled)
  77 "expired ใบของสด": ƒ65,ƒ66; num 267 … 2044 (55 filled)
  78 "waste ใบเตรียมของ": ƒ67,ƒ68; num 0 (55 filled)
  79 "expired ใบเตรียมของ": ƒ69,ƒ70; num 0 … 1756 (55 filled)
  80 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ152; num 840.00 … 3,595.00 fmt #,##0.00 (55 filled)
  81 "ปริมาณทิ้งทั้งหมด (%)": ƒ164; num 0.05 … 0.26 fmt #,##0.00 (55 filled)
  82 "ใบนับของสด-ใบเตรียมของ (กรัม)": ƒ71,ƒ72; num -306.00 … 942.00 fmt #,##0.00 (55 filled)
  83 "ใบนับของสด-ใบเตรียมของ (%)": ƒ165; num -2.18% … 5.64% fmt 0.00% (55 filled)
  84 "ใช้ไปใบเตรียมของ-Item Sales(ก…": ƒ73,ƒ74; num 429.00 … 3,831.00 fmt #,##0.00 (55 filled)
  85 "ใช้ไปใบเตรียมของ-Item Sales (…": ƒ176; num 3.85% … 18.78% fmt 0.00% (55 filled)
  86 "ถั่วแระ (กรัม/1000บาท)": ƒ137; num 47.70 ก. … 85.22 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (55 filled)
  87 "ถั่วแระ ทั้งหมด": ƒ75,ƒ76; num 32,000.00 … 62,000.00 fmt #,##0.00 (55 filled)
  88 "Yield": ƒ144; num 39.34% … 50.54% fmt 0.00% (55 filled)
  89 "waste ใบของสด": ƒ77,ƒ78; num 17088 … 34295 (55 filled)
  90 "expired ใบของสด": ƒ79,ƒ80; num 0 (55 filled)
  91 "waste ใบเตรียมของ": ƒ81,ƒ82; num 0 (55 filled)
  92 "expired ใบเตรียมของ": ƒ83,ƒ84; num 0 … 991 (55 filled)
  93 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ153; num 17,215.00 … 34,295.00 fmt #,##0.00 (55 filled)
  94 "ปริมาณทิ้งทั้งหมด (%)": ƒ166; num 49.46% … 60.66% fmt 0.00% (55 filled)
  95 "ใบนับของสด-ใบเตรียมของ (กรัม)": ƒ85,ƒ86; num 17,088.00 … 34,295.00 fmt #,##0.00 (55 filled)
  96 "ใบนับของสด-ใบเตรียมของ (%)": ƒ167; num 49.46% … 60.66% fmt 0.00% (55 filled)
  97 "ใบเตรียมของ-Item Sales(กรัม)": ƒ87,ƒ88; num 995.00 … 2,966.00 fmt #,##0.00 (55 filled)
  98 "ใบเตรียมของ-Item Sales (%)": ƒ177; num 5.43% … 11.66% fmt 0.00% (55 filled)
  +26 more rows
▸ C126:C138 (13 cells)
  126: C=ไก่ (กรัม/1000บาท)
  127: C=ไก่ ทั้งหมด
  128: C=Yield
  129: C=waste ใบของสด
  130: C=expired ใบของสด
  131: C=waste ใบเตรียมของ
  132: C=expired ใบเตรียมของ
  133: C=ปริมาณทิ้งทั้งหมด (กรัม)
  134: C=ปริมาณทิ้งทั้งหมด (%)
  135–136: C=ใบนับของสด-ใบเตรียมของ …
  137: C=ใบเตรียมของ-Item Sales(…
  138: C=ใบเตรียมของ-Item Sales …
▸ V126:BI138 table 13×40 (520 cells), no header row
  V…BI (40 alike): ƒ140,ƒ117,ƒ147 +10; num -200 … 16,000.00 fmt #,##0.00 (13 filled)
  sample (V…AK of 40):
  126| 24.98 ก. | 28.93 ก. | 11.62 ก. | 15.88 ก. | 14.42 ก. | 22.63 ก. | 22.16 ก. | 21.98 ก. | 21.78 ก. | 25.00 ก. | 22.11 ก. | 24.43 ก. | 22.63 ก. | 28.48 ก. | 25.21 ก. | 34.74 ก.
  127| 16,000.00 | 18,000.00 | 7,000.00 | 9,000.00 | 9,000.00 | 14,000.00 | 14,000.00 | 18,000.00 | 16,000.00 | 21,000.00 | 17,000.00 | 21,000.00 | 19,000.00 | 23,000.00 | 24,000.00 | 30,000.00
  128| 101.25% | 101.11% | 102.86% | 102.22% | 101.78% | 102.43% | 100.86% | 105.44% | 107.75% | 105.50% | 106.94% | 107.26% | 108.46% | 108.03% | 110.43% | 106.13%
  138| 0.00% | 0.55% | 1.39% | 0.00% | -0.66% | 2.44% | 2.69% | 4.74% | 5.19% | 4.01% | 2.58% | 4.63% | 4.21% | 2.83% | 3.28% | 3.45%
▸ V140:BI141 table 2×40 (42 cells), no header row
  V…BF (37 alike): ƒ129; num 0.00 fmt #,##0.00 (1 filled)
  BG -: ƒ129,ƒ194; num 22,000.00 … 99,930.00 fmt #,##0.00 (2 filled)
  BH -: ƒ129,ƒ194; num 17,500.00 … 88,000.00 fmt #,##0.00 (2 filled)
  BI -: ƒ129; num 10,840.00 fmt #,##0.00 (1 filled)
  sample (V…AK of 40):
  140| 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00
  141|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
▸ C142:C160 (19 cells)
  142: C=Others
  143: C=Engawa
  144: C=ก้ามปู
  145: C=กุ้ง
  146: C=ยำสาหร่าย
  147: C=เต้าหู้
  148: C=กิมจิ
  149: C=สลัดออแกนิค
  150: C=จาลาปิโน
  151: C=ต้นหอม
  152: C=กระเจี้ยบ
  153: C=แตงกวา
  154: C=เห็ดออรินจิ
  155: C=มะเขือม่วง
  156: C=หอมแดง
  157: C=กระเทียม
  158: C=มะเขือเทศ
  159: C=ผักชี
  160: C=Mayongchid
▸ G143:BI160 table 18×55 (990 cells), no header row
  G…BI (55 alike): ƒ195,ƒ197,ƒ199 +15; num 0.39 ก. … 22.49 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (18 filled)
  sample (G…V of 55):
  143| 2.50 ก. | 1.82 ก. | 2.65 ก. | 2.63 ก. | 2.12 ก. | 3.59 ก. | 2.71 ก. | 2.78 ก. | 2.07 ก. | 2.09 ก. | 2.12 ก. | 2.20 ก. | 1.57 ก. | 1.95 ก. | 1.82 ก. | 1.54 ก.
  144| 2.61 ก. | 2.44 ก. | 2.43 ก. | 2.18 ก. | 2.88 ก. | 2.67 ก. | 2.73 ก. | 2.52 ก. | 2.13 ก. | 2.00 ก. | 2.08 ก. | 2.07 ก. | 1.49 ก. | 1.48 ก. | 2.47 ก. | 1.40 ก.
  145| 0.39 ก. | 0.39 ก. | 0.35 ก. | 0.43 ก. | 0.31 ก. | 0.41 ก. | 0.33 ก. | 0.39 ก. | 0.33 ก. | 0.36 ก. | 0.37 ก. | 0.32 ก. | 0.30 ก. | 0.30 ก. | 0.28 ก. | 0.26 ก.
  160| 5.76 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.14 ก. | 1.29 ก. | 0.00 ก. | 0.00 ก.
▸ G167:BI168 table 2×55 (110 cells), no header row
  G…BI (55 alike): ƒ231,ƒ232; num 7.66% … 9.19% fmt 0.00% (2 filled)
  sample (G…V of 55):
  167| 9.19% | 9.48% | 11.41% | 7.77% | 13.01% | 7.24% | 8.61% | 10.71% | 9.28% | 9.19% | 5.82% | 12.09% | 6.36% | 2.90% | 3.47% | 3.15%
  168| 7.66% | 10.64% | 6.26% | 7.06% | 8.13% | 11.76% | 9.07% | 7.52% | 8.88% | 10.33% | 9.26% | 8.60% | 3.29% | 1.36% | 0.06% | 0.75%
▸ +5 smaller blocks (21 cells): C21:C27, C42:C48, C166:C168, C244:C246, C140
ƒ formulas (8133 cells, 233 groups, 19 families)
ƒ1 E1:BI1 → =year(E3)
ƒ2 E2:BI2 → =month(E3)
ƒ3 AC3:BI3 → =eomonth(AB3,1)
ƒ4 K4:BI4 → =K5/day(K3)
ƒ5–129 125 similar groups (3510 cells; same shape, refs/constants differ) e.g. G5:AM5 → =sumifs(Sales!$J$3:$J$1000,Sales!$B$3:$B$1000,G$2,Sales!$C$3:$C$1000,G$1); others at AN5:BI5, G6:AM6, AN6:BI6, G7:AM7, AN7:BI7, G9:AM9 +118
ƒ130–140 11 similar groups (535 cells; same shape, refs/constants differ) e.g. G8:BI8 → =G9/G5*1000; others at G29:BI29, G50:BI50, G59:BI59, G66:H66, I66:BI66, G73:BI73 +4
ƒ141–147 G10:BI10 → =(G9-G11)/G9 — same formula in 7 ranges (370 cells): G31:BI31, G75:BI75, G88:BI88, G101:BI101, G114:BI114, V128:BI128; breaks at G11:BI30, G32:BI74, G76:BI87, G89:BI100, G102:BI113 = 5115 different formulas, 330 blank
ƒ148 G13:AM13 → =sumifs('Fish record'!$G$3:$G$1000,'Fish record'!$B$3:$B$1000,G$2,'Fish record'!$C$3:$C$1000,G$1)*-1
ƒ149 AN13:BI13 → =sumifs('Fish record'!$G$3:$G$2000,'Fish record'!$B$3:$B$2000,AN$2,'Fish record'!$C$3:$C$2000,AN$1)*-1
ƒ150–156 7 similar groups (370 cells; same shape, refs/constants differ) e.g. G14:BI14 → =sum(G11:G13); others at G35:BI35, G80:BI80, G93:BI93, G106:BI106, G119:BI119, V133:BI133
ƒ157–173 17 similar groups (905 cells; same shape, refs/constants differ) e.g. G15:BI15 → =G14/G9; others at G17:BI17, G36:BI36, G38:BI38, G56:BI56, G65:BI65, G72:BI72 +10
ƒ174–180 7 similar groups (370 cells; same shape, refs/constants differ) e.g. G19:BI19 → =G18/(G9-G11-G12); others at G40:BI40, G85:BI85, G98:BI98, G111:BI111, G124:BI124, V138:BI138
ƒ181–184 4 similar groups (220 cells; same shape, refs/constants differ) e.g. G22:BI22 → =(G23+G24)/G$6*1000; others at G25:BI25, G43:BI43, G46:BI46
ƒ185–186 G23:BI23 → =G21-G26 — same formula in 2 ranges (110 cells): G44:BI44; breaks at G24:BI43 = 990 different formulas, 110 blank
ƒ187–190 4 similar groups (220 cells; same shape, refs/constants differ) e.g. G24:BI24 → =(G23/G21)*G14; others at G27:BI27, G45:BI45, G48:BI48
ƒ191–193 G52:BI52 → =(G51-G55-G54)/G51 [2 errors] — same formula in 3 ranges (165 cells): G61:BI61, G68:BI68; breaks at G53:BI60, G62:BI67 = 660 different formulas, 110 blank
ƒ194 BG141:BH141 → =sum(BG127,BG140)
ƒ195–230 36 similar groups (990 cells; same shape, refs/constants differ) e.g. G143:AM143 → =SUMIFS(Others!$J$3:$J$1000,Others!$B$3:$B$1000,G$2,Others!$C$3:$C$1000,G$1)/G$5*1000; others at AN143:BI143, G144:AM144, AN144:BI144, G145:AM145, AN145:BI145, G146:AM146 +29
ƒ231 G167:BI167 → =G12/(G9-G11)
ƒ232 G168:BI168 → =G33/(G30-G32)
ƒ233 C244:C246 ↓ ='Item Sales (POS)'!C448
◆ formatting
fg#434343 italic (3919 cells): C10:BI10, C14:BI20, C23:C24, C26:C28 +33
italic (1984 cells): C11:BI13, C32:BI34, C53:D54, C62:C63 +11
bg#FFF2CC (649 cells): C50:BI51, C66:BI67, C73:BI74, C99:BI100 +2
bg#C9DAF8 (236 cells): C5:BI7, C142:BI142
bg#D9EAD3 (236 cells): C59:BI60, C86:BI87
bg#EFEFEF bold (177 cells): C1:BI3
bg#F9CB9C (120 cells): C112:BI113, D22, D25
bg#FCE5CD (119 cells): C8:BI9, D21
+7 more styles

## 4. Monthly Summary (item sales) · B1:CT358 · 14k cells · frozen 0r3c
reads: Item Sales (POS) (96g)
▸ C1:BB3 row-oriented 3×52 (153 cells), labels in column C
  1 "Year": ƒ1; num 2022 … 2026 (50 filled)
  2 "Month": ƒ2,ƒ3; num 1 … 12 (50 filled)
  3 "Date": date 3/31/2022 … 4/30/2026 (50 filled)
▸ C5:BB32 row-oriented 28×52 (1172 cells), labels in column C
  5 -: ƒ4,ƒ5; num -27.89% … 18.97% fmt 0.00% (34 filled)
  6 "In-store": ƒ28; num 296 … 868 (50 filled)
  7 "The Classic": ƒ42; num 62.5 … 292 (50 filled)
  8 "BYO": ƒ43; num 63 … 148 (50 filled)
  9 "X Factor": ƒ44,ƒ45; num 46 … 146 (50 filled)
  10 "California": ƒ46; num 1.5 … 176 (50 filled)
  11 "Sexy Salmon": ƒ47; num 9 … 75.5 (50 filled)
  12 "K bowl": ƒ48; num 15 … 55 (50 filled)
  13 "Triple sea": ƒ49,ƒ50; num 0 … 29 (50 filled)
  14 "Pimpim pesto": ƒ51,ƒ52; num 0 … 12 (50 filled)
  15 "Chipotle": ƒ53; num 3 … 37.5 (50 filled)
  16 "Hunter's bowl": ƒ54; num 2 … 16 (50 filled)
  17 "Tofu emperor": ƒ55; num 1 … 30 (50 filled)
  18 "Avo-mango balsamic": ƒ56; num 1 … 25 (50 filled)
  19 "Triple sea (no engawa)": ƒ57; num 0 (50 filled)
  20 "Chicken bowl": ƒ29; num 0 … 125 (35 filled)
  21 "instore Chicken Rosa": ƒ127; num 0 … 31 (35 filled)
  22 "instore Chicken Black Truffle": ƒ127; num 0 … 40 (35 filled)
  23 "instore Chicken Tom Yum": ƒ127; num 0 … 60 (35 filled)
  24 "Wrap": ƒ30,ƒ31,ƒ32; num 15 … 141 (31 filled)
  25 "IN Chicken Chipotle Wrap ": ƒ129,ƒ130,ƒ131 +1; num 4 … 68 (33 filled)
  26 "IN CBT Wrap ": ƒ129,ƒ130,ƒ131 +1; num 0 … 31 (33 filled)
  27 "IN Rainbow Wrap": ƒ129,ƒ130,ƒ131 +1; num 0 … 25 (33 filled)
  28 "IN kimchi wrap": ƒ129,ƒ130,ƒ131 +1; num 0 … 47 (33 filled)
  29 "IN Guacamole wrap": ƒ129,ƒ130,ƒ131 +1; num 0 … 31 (33 filled)
  30 "IN Caesar wrap": ƒ130,ƒ131,ƒ128; num 2 … 24 (27 filled)
  31 "IN Chicken Rosa Wrap": ƒ131,ƒ128; num 0 … 16 (24 filled)
  32 "IN Chicken Seafood Pesto Wrap": ƒ131,ƒ128; num 0 … 7 (24 filled)
▸ BN5:CT29 table 25×33 (524 cells), no header row
  BN…BR (5 alike): ƒ12,ƒ72,ƒ86; num 0.00% … 659.5 fmt 0.00% (16 filled)
  BT -: ƒ14; num 0 … 1886 (24 filled)
  BU -: ƒ6,ƒ14; num 0 … 1901.5 fmt 0.00% (25 filled)
  BV -: ƒ6,ƒ14; num 0 … 1966.5 fmt 0.00% (25 filled)
  BW -: ƒ6; num -2.31% … 1921 fmt 0.00% (2 filled)
  BX -: ƒ15,ƒ77,ƒ91; num 0.00% … 1968 fmt 0.00% (16 filled)
  BY -: ƒ15,ƒ78,ƒ92; num 0.00% … 2048.5 fmt 0.00% (16 filled)
  BZ -: ƒ15,ƒ79,ƒ93; num 0.00% … 2209.5 fmt 0.00% (16 filled)
  CA -: text 14 distinct e.g. "Avo-mango balsamic", "BYO", "California" (14 filled)
  CB…CJ (9 alike): num 0.00% … 22.18% fmt 0.00% (14 filled)
  CL…CO (4 alike): ƒ9,ƒ80,ƒ94; num 0.00% … 488 fmt 0.00% (16 filled)
  CP -: text 14 distinct e.g. "Avo-mango balsamic", "BYO", "California" (14 filled)
  CQ…CT (4 alike): num 0.00% … 21.41% fmt 0.00% (14 filled)
  sample (BN…CC of 33):
  5|  |  |  |  |  |  |  | 0.82% | 3.42% | -2.31% |  |  |  |  |  | 
  6| 659.5 | 726 | 688.5 | 726.5 | 649 |  | 1886 | 1901.5 | 1966.5 | 1921 | 1968 | 2048.5 | 2209.5 |  |  | 
  7| 19.03% | 18.46% | 17.94% | 18.24% | 19.88% |  | 436.5 | 420.5 | 408 |  | 22.18% | 20.53% | 18.47% | The Classic | 22.18% | 20.53%
  29|  |  |  |  |  |  | 0 | 0 | 0 |  |  |  |  |  |  | 
▸ BH6:BI20 (30 cells)
  6: BH=ƒ10→653 BI=ƒ11→674
  7: BH=ƒ70→18.76% BI=ƒ71→19.21%
  8: BH=ƒ70→17.92% BI=ƒ71→16.91%
  9: BH=ƒ70→17.69% BI=ƒ71→21.66%
  10: BH=ƒ70→10.57% BI=ƒ71→10.09%
  11: BH=ƒ70→2.22% BI=ƒ71→2.74%
  12: BH=ƒ70→3.98% BI=ƒ71→6.45%
  13: BH=ƒ70→2.76% BI=ƒ71→3.71%
  14: BH=ƒ70→0.00% BI=ƒ71→0.00%
  15: BH=ƒ70→2.37% BI=ƒ71→3.19%
  16: BH=ƒ70→1.53% BI=ƒ71→0.74%
  17: BH=ƒ70→1.07% BI=ƒ71→0.59%
  18: BH=ƒ70→1.68% BI=ƒ71→1.04%
  19: BH=ƒ70→0.00% BI=ƒ71→0.00%
  20: BH=ƒ70→2.45% BI=ƒ71→2.08%
▸ C34:BB60 row-oriented 27×52 (1120 cells), labels in column C
  34…51 (18 alike) "" "Delivery" "The Classic" "BYO" "X Factor" "California" "Sexy Salmon" "K bowl" "Triple sea" "Pimpim pesto" "Chipotle" "Hunter's bowl" "Tofu emperor" "Avo-mango balsamic" "Chicken bowl" "Deli Chicken Rosa" "Deli Chicken Black Truffle" "De…: ƒ7; num -23.70% … 39.10% fmt 0.00% (31 filled)
  52 "Wrap": ƒ35,ƒ36; num 0 … 393 (33 filled)
  53 "Chicken Chipotle Wrap ": ƒ24,ƒ25,ƒ26 +1; num 0 … 162 (33 filled)
  54 "CBT Wrap ": ƒ24,ƒ25,ƒ26 +1; num 0 … 54 (33 filled)
  55 "Rainbow Wrap": ƒ24,ƒ25,ƒ26 +1; num 0 … 84 (33 filled)
  56 "Kimchi wrap": ƒ24,ƒ25,ƒ26 +1; num 0 … 64 (33 filled)
  57 "Guacamole wrap": ƒ24,ƒ25,ƒ26 +1; num 0 … 32 (33 filled)
  58 "Caesar wrap": ƒ25,ƒ26,ƒ132; num 8 … 52 (27 filled)
  59 "Chicken Rosa Wrap": ƒ26,ƒ132; num 2 … 23 (24 filled)
  60 "Chicken Seafood Pesto Wrap": ƒ26,ƒ132; num 0 … 8 (24 filled)
▸ BN34:CT57 table 24×33 (509 cells), no header row
  BN…BR (5 alike): ƒ19,ƒ100,ƒ114; num 0.00% … 1423.5 fmt 0.00% (15 filled)
  BT -: ƒ21; num 0 … 2254 (23 filled)
  BU -: ƒ8,ƒ21; num 0 … 2338.5 fmt 0.00% (24 filled)
  BV -: ƒ8,ƒ21; num -2.33% … 2284 fmt 0.00% (24 filled)
  BW -: ƒ8; num -3.87% … 2195.5 fmt 0.00% (2 filled)
  BX -: ƒ22,ƒ105,ƒ119; num 0.00% … 2359 fmt 0.00% (15 filled)
  BY -: ƒ22,ƒ106,ƒ120; num 0.00% … 2563.5 fmt 0.00% (15 filled)
  BZ -: ƒ22,ƒ107,ƒ121; num 0.00% … 2617 fmt 0.00% (15 filled)
  CA -: text 14 distinct e.g. "Avo-mango balsamic", "BYO", "California" (14 filled)
  CB…CJ (9 alike): num 0.00% … 25.69% fmt 0.00% (13 filled)
  CL…CO (4 alike): ƒ16,ƒ108,ƒ122; num 0.00% … 783.5 fmt 0.00% (15 filled)
  CP -: text 14 distinct e.g. "Avo-mango balsamic", "BYO", "California" (14 filled)
  CQ…CT (4 alike): num 0.00% … 23.17% fmt 0.00% (13 filled)
  sample (BN…CC of 33):
  34|  |  |  |  |  |  |  | 3.75% | -2.33% | -3.87% |  |  |  |  |  | 
  35| 1423.5 | 1236 | 1069 | 1275 | 1175.5 |  | 2254 | 2338.5 | 2284 | 2195.5 | 2359 | 2563.5 | 2617 |  |  | 
  36| 19.70% | 21.72% | 24.09% | 24.12% | 22.50% |  | 606 | 683.5 | 633.5 |  | 25.69% | 26.66% | 24.21% | The Classic | 25.69% | 26.66%
  57|  |  |  |  |  |  | 0 | 0 | 0 |  |  |  |  |  |  | 
▸ B62:BB103 table 42×53 (2122 cells), no header row
  B -: num 1 … 1.5 (39 filled)
  C -: text 40 distinct e.g. "Avo-Mango Balsamic Sala…", "Avo-Mango Balsamic Sala…", "Avo-Mango Balsamic Sala…" (40 filled)
  E…BB (50 alike): ƒ135,ƒ136,ƒ137 +5; num 0 … 120 (39 filled)
  sample (B…Q of 53):
  62|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  63|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  64|  | Instore |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  103| 1.5 | Mayongchid |  | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
▸ B105:BB140 table 36×53 (1821 cells), no header row
  B -: num 1 … 1.5 (35 filled)
  C -: text 36 distinct e.g. "Avo-mango Deli - พิเศษ", "Avo-mango Deli - สแตนดา…", "BYO Deli" (36 filled)
  E…BB (50 alike): ƒ143,ƒ144,ƒ145 +2; num 0 … 277 (35 filled)
  sample (B…Q of 53):
  105|  | Delivery |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  106| 1 | Classic Deli - สแตนดาร์ด |  | 277 | 292 | 215 | 185 | 137 | 153 | 172 | 167 | 153 | 163 | 203 | 152 | 195
  107| 1 | Classic Deli - พิเศษ |  | 91 | 117 | 79 | 76 | 54 | 67 | 68 | 61 | 50 | 41 | 83 | 30 | 45
  140| 1 | Mayongchid Deli |  | 25 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1
▸ C142:BB285 table 144×52 (5005 cells), no header row
  C -: text 143 distinct e.g. "Tuna Salsa (1-11/9)", "Buy1 Get1 Free SET 1 : …", "Buy1 Get1 Free SET 2 : …" (144 filled)
  E…BB (50 alike): ƒ148,ƒ149,ƒ164 +4; num 0 … 220 (27 filled)
  sample (C…R of 52):
  142| Promotion |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  143| Trip + KBowl (FPD 10.1-… |  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  144| Cali + Hun (FPD 10.1-31… |  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  285| Seafoof pesto Wrap 12" |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
▸ C287:C336 table 50×1 (50 cells), no header row
  C -: text 50 distinct e.g. "Chicken Black Truffle S…", "Chicken Kimchi Set", "Chicken Rosa Set" (50 filled)
  sample (C…C):
  287| GF Chicken Chipotle Wrap
  288| GF CBT Wrap
  289| GF Rainbow Wrap
  336| x factor chicken
▸ X287:BB336 table 50×31 (1121 cells), no header row
  X…BB (31 alike): ƒ205,ƒ210,ƒ214; num 2 … 14 (9 filled)
  sample (X…AM of 31):
  287| 9 | 28 | 20 | 33 | 14 | 28 | 33 | 33 | 30 | 41 | 67 | 126 | 91 | 70 | 74 | 88
  288| 14 | 24 | 18 | 22 | 16 | 16 | 25 | 23 | 21 | 19 | 16 | 19 | 17 | 26 | 19 | 17
  289| 12 | 14 | 7 | 13 | 15 | 26 | 16 | 20 | 28 | 36 | 32 | 31 | 28 | 29 | 27 | 29
  336|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
▸ Z338:BB358 table 21×29 (599 cells), no header row
  Z…BB (29 alike): ƒ225,ƒ226,ƒ228; num 0 … 5 (16 filled)
  sample (Z…AO of 29):
  338| 0 | 0 | 2 | 21 | 33 | 23 | 11 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  339| 5 | 3 | 6 | 5 | 5 | 2 | 1 | 4 | 4 | 3 | 4 | 5 | 2 | 2 | 2 | 2
  340| 0 | 0 | 4 | 7 | 13 | 18 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  358|  | 3 | 3 | 2 | 5 | 4 | 1 | 1 | 7 | 0 | 3 | 3 | 1 | 4 | 3 | 2
▸ +9 smaller blocks (140 cells): BH35:BI48, BP287:BV291, BP303:BV307, BP311:BV315, C338:C358, BR273:BV274, BR327:BV327, BH24:BI24 +1
ƒ formulas (13k cells, 239 groups, 17 families)
ƒ1 E1:BB1 → =year(E3)
ƒ2–3 E2:AV2 → =month(E3) — same formula in 2 ranges (48 cells): AY2:BB2; breaks at AW2:AX2 = 2 hardcoded values
ƒ4 T5:U5 → =T20/(T6+T20)
ƒ5–8 W5:BB5 → =(W6-V6)/V6 — same formula in 4 ranges (69 cells): BU5:BW5, X34:BB34, BU34:BW34; breaks at BU6:BW33 = 1 hardcoded values, 48 different formulas, 35 blank
ƒ9–27 19 similar groups (405 cells; same shape, refs/constants differ) e.g. CL5:CO5 → =W6+W20+W24; others at BH6, BI6, BN6, BO6:BR6, BT6:BV29, BX6:BZ6 +12
ƒ28–41 14 similar groups (273 cells; same shape, refs/constants differ) e.g. E6:BB6 → =sum(E7:E18); others at T20:BB20, X24:AA24, AB24:AD24, AE24:BB24, E35:BB35, T48:BB48 +7
ƒ42–69 28 similar groups (1250 cells; same shape, refs/constants differ) e.g. E7:BB7 → =sumproduct($B65:$B67,E65:E67); others at E8:BB8, E9:Z9, AA9:BB9, E10:BB10, E11:BB11, E12:BB12 +21
ƒ70–126 57 similar groups (437 cells; same shape, refs/constants differ) e.g. BH7:BH20 ↓ =AG7/$BH$6; others at BI7:BI20, BN7:BN20, BO7:BO20, BP7:BP20, BQ7:BQ20, BR7:BR20 +50
ƒ127 T21:BB23 ▦ =T236+T239
ƒ128 AK25:BB32 ▦ =AK270+AK278
ƒ129–131 V25:AA29 ▦ =V270 — same formula in 3 ranges (96 cells): AB25:AD30, AE25:AJ32
ƒ132 AK53:BB60 ▦ =AK287+AK303+AK311+AK295+AK319
ƒ133 T253:BB255 ▦ =T236+T239+T245+T249+T242
ƒ134 X63:BB63 → =X67+X70+X76+X73+X79+X82+X85+X88+X91+X94+X97+X100+X103
ƒ135–229 95 similar groups (10030 cells; same shape, refs/constants differ) e.g. E65:BB67 ▦ =sumifs('Item Sales (POS)'!7:7,'Item Sales (POS)'!$2:$2,E$2,'Item Sales (POS)'!$1:$1,E$1); others at E68:BB70, E71:BB73, E74:BB76, E77:BB79, E80:BB82, E83:BB102 +88
ƒ230 R236:R241 ↓ =U236*250
ƒ231 S239:S241 ↓ =T239*125
ƒ232 S236:S238 ↓ =250*T236
ƒ233 V258:BB260 ▦ =sumifs('Item Sales (POS)'!314:314,'Item Sales (POS)'!$2:$2,V$2,'Item Sales (POS)'!$1:$1,V$1)+sumifs('Item Sales (POS)'!52:52,'Item Sales (POS)'!$2:$2,V$2,'Item Sales (POS)'!$1:$1,V$1)
ƒ234–239 6 similar groups (60 cells; same shape, refs/constants differ) e.g. BR287:BR291 ↓ =X287*$BP287; others at BT287:BV291, BR303:BR307, BT303:BV307, BR311:BR315, BT311:BV315
◆ formatting
bg#D9EAD3 (7082 cells): B105:B140, B143:CT144, B148:CT148, B153:CT154 +14
bg#FFF2CC (6340 cells): B65:CT103, B155:CT157, B236:Q238, B270:BP285 +11
bg#F4CCCC (2718 cells): B20:BC20, B24:BC24, B48:BC48, B52:BC52 +22
bg#EAD1DC (2676 cells): B209:CT211, B220:CT221, B249:CT251, B257:CT257 +7
bg#D0E0E3 (2069 cells): B212:CT214, B222:CT223, B229:CT230, B264:CT266 +6
bg#FCE5CD (1729 cells): B162:CT162, B175:D180, B215:CT217, B239:Q241 +4
bg#CFE2F3 (663 cells): B245:CT247, C347:C358, Z347:BC358
bg#E6B8AF (388 cells): B149:CT150, B158:CT159
+8 more styles
hidden rows: 181-200

## 5. Item Sales (Cal) · A1:BPD1148 · 1.0M cells · frozen 5r4c
reads: Item Sales (POS) (261g)
read by: Monthly Summary (Usage), Item Sales (Usage)
▸ A1:BPD662 table 662×1772 (1008977 cells), no header row
  A -: ƒ4; num 1 … 660 (660 filled)
  B -: num 24  … 60  fmt #,##0_);[Red](#,##0); enum s(79) t(32) st(20) (139 filled)
  C -: text 464 distinct e.g. "Chipotle K-Mix (FPD onl…", "Hunter’s Bowl - พิเศษ", "Hunter’s Bowl - สแตนดาร…" (628 filled)
  D -: num -45 … 200 fmt #,##0_);[Red](#,##0) (329 filled)
  E…BPD (1768 alike): ƒ1,ƒ2,ƒ5 +87; date 1-Mar-22; num (48) … 2535; 2 errors (510 filled)
  sample (A…P of 1772):
  1|  |  |  |  | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022
  2|  |  |  |  | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3
  3| 1 |  | วันที่ |  | 1-Mar-22 | 2-Mar-22 | 3-Mar-22 | 4-Mar-22 | 5-Mar-22 | 6-Mar-22 | 7-Mar-22 | 8-Mar-22 | 9-Mar-22 | 10-Mar-22 | 11-Mar-22 | 12-Mar-22
  662| 660 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
▸ IZ1147:IZ1148 (2 cells)
  1147: IZ=ƒ85→48
  1148: IZ=ƒ86→0
ƒ formulas (1.0M cells, 306 groups, 22 families)
ƒ1 E1:BPD1 → =year(E3)
ƒ2 E2:BPD2 → =month(E3)
ƒ3 F3:BPD3 → =E3+1
ƒ4 A6:A662 ↓ =A5+1
ƒ5–86 82 similar groups (790333 cells; same shape, refs/constants differ) e.g. E6:BPD54 ▦ ='Item Sales (POS)'!E6*$D6; others at E57:BPD61, E62:BPD65, E68:BPD69, E70:BPD115, E116:BPD116, E117:HK118 +75
ƒ87–157 71 similar groups (120072 cells; same shape, refs/constants differ) e.g. VL55:BPD55 → =('Item Sales (POS)'!VL65+'Item Sales (POS)'!VL57)*$D55; others at VU178:WX178, VU196:WX196, AKS209:BPD209, E452:BPD452, E456:BPD458, E460:BPD460 +64
ƒ158–182 25 similar groups (10473 cells; same shape, refs/constants differ) e.g. AKS56:BPD56 → =('Item Sales (POS)'!AKS75)*$D56; others at ARZ210:BPD211, FD473:BPD473, AXD532:BCA532, FD577:BPD577, FD597:BPD597, XU611:BPD611 +18
ƒ183 E66:BPD67 ▦ =#REF!*$D66 [#REF!] [3536 errors]
ƒ184–186 3 similar groups (2235 cells; same shape, refs/constants differ) e.g. HL117:BPD117 → ='Item Sales (POS)'!HL199+'Item Sales (POS)'!HL164*$D117; others at BCB553:BPD553, BCB554:BPD554
ƒ187–209 23 similar groups (13111 cells; same shape, refs/constants differ) e.g. VL208:BPD208 → =('Item Sales (POS)'!VL218+'Item Sales (POS)'!VL272+'Item Sales (POS)'!VL302+'Item Sales (POS)'!VL378)*$D208; others at E471:BPD471, GG525:BPD525, E529:BPD529, AWY531, AYN531, AYP531 +16
ƒ210 E444:BPD446 ▦ =sumifs(E$6:E$440,$B$6:$B$440,$B444)
ƒ211 E447:BPD447 → =E444+E446
ƒ212 E448:BPD448 → =E445+E446
ƒ213–246 34 similar groups (12512 cells; same shape, refs/constants differ) e.g. E451:FC451 → =sum(E452:E471); others at FD451:IY451, IZ451:AXC451, AXD451:BPD451, E479:IY479, IZ479:APM479, APN479:AXC479 +27
ƒ247–255 9 similar groups (12159 cells; same shape, refs/constants differ) e.g. PM459:BPD459 → =('Item Sales (POS)'!PM22*$D459); others at PM463:BPD463, PM490:BPD490, OR515:BPD515, OR540:BPD540, PM544:BPD544, OR561:BPD561 +2
ƒ256 E461:BPD462 ▦ =('Item Sales (POS)'!E24+ 'Item Sales (POS)'!E330)*$D461
ƒ257–265 9 similar groups (2664 cells; same shape, refs/constants differ) e.g. VU465:XD465 → =('Item Sales (POS)'!VU28+'Item Sales (POS)'!VU336+'Item Sales (POS)'!VU303)*$D465; others at BDM476, VU492:XD492, VU546:XD546, VU591:XD591, ZX610:BPD610, AKS613:ATY614 +2
ƒ266–276 11 similar groups (499 cells; same shape, refs/constants differ) e.g. AQQ511:ARY511 → =('Item Sales (POS)'!AQX79+'Item Sales (POS)'!AQX231+'Item Sales (POS)'!AQX288-'Item Sales (POS)'!AQX427)*$D531; others at AKS531:AWX531, AWZ531:AYM531, AYO531, AYQ531:AYW531, AYY531:AZG531, AZI531:BAG531 +4
ƒ277–283 7 similar groups (2384 cells; same shape, refs/constants differ) e.g. AHE530:AVX530 → =('Item Sales (POS)'!AHE62+'Item Sales (POS)'!AHE70+'Item Sales (POS)'!AHE223+'Item Sales (POS)'!AHE280+'Item Sales (POS)'!AHE309)*$D530; others at QT601:TO603, TQ601:UG603, AHE605:ATV605, XU606:ZI606, AAE607:ADP607, ADQ607:ATV609
ƒ284–295 12 similar groups (5984 cells; same shape, refs/constants differ) e.g. AVY530:BPD530 → =('Item Sales (POS)'!AVY62+'Item Sales (POS)'!AVY70+'Item Sales (POS)'!AVY223+'Item Sales (POS)'!AVY280+'Item Sales (POS)'!AVY309+'Item Sales (POS)'!AVY398)*$D530; others at TP601:TP603, UH601:ATV603, VL604:ZI605, AHE604:ATV604, ZJ605:AHD605, ATW605:BPD605 +5
ƒ296–301 6 similar groups (2973 cells; same shape, refs/constants differ) e.g. ATW601:BPD601 → =('Item Sales (POS)'!ATW52+'Item Sales (POS)'!ATW314+'Item Sales (POS)'!ATW210+'Item Sales (POS)'!ATW267+'Item Sales (POS)'!ATW296+'Item Sales (POS)'!ATW369+'Item Sales (POS)'!ATW407)*$D601; others at ATW602:BPD602, ATW603:BPD603, ZJ604:AHD604, ATW604:BPD604, ATW606:BPD606
ƒ302 E642:BPD642 → =iferror(sumifs(E$160:E$194,$B$160:$B$194,$B642)+E645+(E644/2),0)
ƒ303 E643:BPD643 → =iferror(sumifs(E$160:E$194,$B$160:$B$194,$B643)+E646+(E644/2),0)
ƒ304 E644:BPD644 → =sumifs(E$160:E$195,$B$160:$B$195,$B644)+E647
ƒ305 E645:BPD645 → =sumifs(E$224:E$243,$B$224:$B$243,$B645) * ('Item Sales (POS)'!E348/('Item Sales (POS)'!E348+'Item Sales (POS)'!E50)) [71 errors]
ƒ306 E646:BPD646 → =sumifs(E$224:E$243,$B$224:$B$243,$B646) * ('Item Sales (POS)'!E348/('Item Sales (POS)'!E348+'Item Sales (POS)'!E50)) [71 errors]
◆ formatting
bg#FFF2CC (272168 cells): A1:A1148, B6:BPD9, B43:BPD45, B51:BPD54 +8
bg#EFEFEF (164890 cells): B207:GF207, B210:BPD223, B362:BPD440
bg#F9CB9C (120428 cells): B10:BPD13, B40:BPD42, B186:BPD187, B224:BPD264 +5
bg#F4CCCC (76169 cells): B1:C4, B18:BPD21, B117:BPD133, B163:BPD165 +11
bg#D9EAD3 (56672 cells): B34:BPD36, B55:BPD56, B166:BPD167, B180:BPD182 +5
bg#CFE2F3 (42317 cells): B14:BPD17, B57:BPD69, B174:BPD176, B204:GF206 +1
bg#FCE5CD (19481 cells): B22:BPD25, B116:BPD116, B160:BPD162, B265:BPD266 +1
bg#D9D2E9 (15939 cells): B26:BPD29, B177:BPD179, B191:BPD192
+8 more styles
hidden rows: 57-73, 78-115, 213-221, 270-276, 363-439
hidden cols: F-KX, KZ-YY, ZA-ANA, ANC-BBB

## 6. Item Sales (POS) · A1:BMW874 · 997k cells · frozen 5r4c
read by: Monthly Summary (Usage), Monthly Summary (item sales), Item Sales (Cal)
▸ C1:BMW4 table 4×1711 (6836 cells), no header row
  C -: C3=วันที่
  E -: date 1-Mar-22; num 3 (3 filled)
  F…BMW (1708 alike): ƒ1,ƒ2,ƒ3; date 2-Mar-22; num 3 … 2022 (4 filled)
  sample (C…R of 1711):
  1|  |  |  | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022
  2|  |  | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3
  3| วันที่ |  | 1-Mar-22 | 2-Mar-22 | 3-Mar-22 | 4-Mar-22 | 5-Mar-22 | 6-Mar-22 | 7-Mar-22 | 8-Mar-22 | 9-Mar-22 | 10-Mar-22 | 11-Mar-22 | 12-Mar-22 | 13-Mar-22 | 14-Mar-22
  4|  |  | Tue,  ,  | Wed,  ,  | Thu,  ,  | Fri,  ,  | Sat,  ,  | Sun,  ,  | Mon,  ,  | Tue,  ,  | Wed,  ,  | Thu,  ,  | Fri,  ,  | Sat,  ,  | Sun,  ,  | Mon,  , 
▸ A5:BMW874 table 870×1713 (989711 cells), no header row
  A -: ƒ4,ƒ5,ƒ6; num 1 … 862 (870 filled)
  C -: text 718 distinct e.g. "ข้าวสาหร่าย", "ผักดอง", "มะม่วง" (766 filled)
  D -: text 3 distinct e.g. "แทนโดย", "ไม่รับ", "ไม่รับท้อปปิ้ง" (3 filled)
  E…BS (67 alike): num 0 … 37 (402 filled)
  BT -: num 0 … 65; text "7 May" (415 filled)
  BU…BX (4 alike): num 0 … 21 (414 filled)
  BY -: num 0 … 55; text "12 May" (416 filled)
  BZ -: num 0 … 66; text "13 May" (416 filled)
  CA -: num 0 … 35; text "14 May" (416 filled)
  CB -: num 0 … 46; text "15 May" (418 filled)
  CC…BMW (1633 alike): num 0 … 27 (416 filled)
  sample (A…P of 1713):
  5| 1 |  | CSD |  |  |  |  |  |  |  |  |  |  |  |  | 
  6| 2 |  | The Classic เล็ก |  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  7| 3 |  | The Classic - สแตนดาร์ด |  | 2 | 3 | 2 | 3 | 5 | 5 | 5 | 2 | 1 | 3 | 7 | 9
  874| 862 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
ƒ formulas (11k cells, 11 groups, 6 families)
ƒ1 F1:BMW1 → =year(E3)
ƒ2 E2:BMW2 → =month(E3)
ƒ3–6 4 similar groups (2577 cells; same shape, refs/constants differ) e.g. F3:BMW3 → =E3+1; others at A6:A288, A289, A290:A874
ƒ7–9 E775:AAN776 ▦ =E573+E577+E581+E585+E589+E593+E597+E601+E605+E609 — same formula in 3 ranges (3417 cells): AAP775:BMW776, AAO776; breaks at AAO775:AAO776 = 2 different formulas
ƒ10 AAO775 =AAO573+AAO577+AAO581+AAO585+AAO589+AAO593+AAO597+AAO601+AAO605+AAO609+AAO556
ƒ11 E777:BMW777 → =E612
◆ formatting
bg#FFF2CC (305357 cells): A6:A316, A318:A874, B6:ANF9, B43:ANF45 +54
bg#EFEFEF (292233 cells): B361:ANF361, B447:ANF512, B671:B771, B775:ANF777 +17
bg#F4CCCC (266610 cells): A1:A5, B1:C4, B18:ANF21, B164:ANF164 +48
bg#D9EAD3 (174318 cells): B34:ANF36, B55:ANF72, B201:ANF231, B289:ANF309 +24
bg#F9CB9C (117921 cells): B10:ANF13, B40:ANF42, B154:ANF154, B344:ANF345 +16
bg#CFE2F3 (68360 cells): B14:ANF17, B73:ANF105, B332:ANF334, ANH14:ANL17 +8
bg#D9D2E9 (29053 cells): B26:ANF29, B335:ANF337, B349:ANF358, ANH26:ANL29 +8
bg#FCE5CD (27344 cells): B22:ANF25, B318:ANF320, B438:ANF446, ANH22:ANL25 +8
+23 more styles
hidden cols: F-KX, KZ-YY, ZA-ANA, ANC-BBB

## 7. Item Sales (Usage) · A1:M2000 · 24k cells · frozen 2r1c
reads: Item Sales (Cal) (10g)
read by: Salmon, Tuna, Chicken, ChickenBB, Avocado, Mango, MahachanokMango, Pomelo, Tamago, Edamame, Ikura
▸ A1:M2000 table 2000×13 (24335 cells), header row 2
  A "[1] รายการ": date Tue, Mar 1, 2022 … Wed, Aug 18, 2027; text "Date" (1998 filled)
  B "[2] ": ƒ2; num 1 … 12; text "Month" (1998 filled)
  C "[3] ": ƒ3; num 2022 … 2027; text "Year" (1998 filled)
  D "[4] Salmon": ƒ4; num 0 … 4140 (1997 filled)
  E "[5] Tuna": ƒ5; num -15 … 2560 (1997 filled)
  F "[6] Mango": ƒ6; num 0  … 2,046  fmt #,##0_);[Red](#,##0) (1997 filled)
  G "[7] Avocado": ƒ7; num 0 … 2004 (1997 filled)
  H "[8] Pomelo": ƒ8; num 0  … 1,300  fmt #,##0_);[Red](#,##0) (1997 filled)
  I "[9] Tamago": ƒ9; num 0  … 1,728  fmt #,##0_);[Red](#,##0) (1997 filled)
  J "[10] Edamame": ƒ10; num 0  … 1,408  fmt #,##0_);[Red](#,##0) (1997 filled)
  K "[11] Ikura": ƒ11; num 0 … 312 (1997 filled)
  L "[12] Chicken": ƒ12; num 0 … 4270 (1540 filled)
  M "[13] Chicken BB": ƒ13; num 0 … 2000 (801 filled)
  sample (A…M):
  3| Date | Month | Year |  |  |  |  |  |  |  |  |  | 
  4| Tue, Mar 1, 2022 | 3 | 2022 | 2535 | 975 | 1,254  | 684 | 760  | 790  | 836  | 208 |  | 
  5| Wed, Mar 2, 2022 | 3 | 2022 | 3315 | 1125 | 1,078  | 768 | 540  | 1,052  | 990  | 201 |  | 
  2000| Wed, Aug 18, 2027 | 8 | 2027 | 0 | 0 | 0  | 0 | 0  | 0  | 0  | 0 | 0 | 0
ƒ formulas (22k cells, 13 groups, 4 families)
ƒ1 B1:M1 → =A1+1
ƒ2 B4:B2000 ↓ =month(A4)
ƒ3 C4:C2000 ↓ =year(A4)
ƒ4–13 10 similar groups (18317 cells; same shape, refs/constants differ) e.g. D4:D2000 ↓ =hlookup($A4,'Item Sales (Cal)'!$3:656,'Item Sales (Cal)'!A$447,1); others at E4:E2000, F4:F2000, G4:G2000, H4:H2000, I4:I2000, J4:J2000 +3
◆ formatting
bg#FFF2CC (5994 cells): F3:F2000, H3:H2000, L3:L2000
bg#D9EAD3 (4797 cells): G3:G2000, J3:J2000, M1200:M2000
bg#F9CB9C (1998 cells): D3:D2000
bg#F4CCCC (1998 cells): E3:E2000
bg#FFE599 (1998 cells): I3:I2000
bg#FCE5CD (1998 cells): K3:K2000
bg#D9EAD3 bold (1200 cells): G2, J2, M2:M1199
italic (13 cells): A1:M1
+7 more styles

## 8. Monthly Inventory Summary · A1:AE35 · 941 cells · frozen 0r2c
reads: Salmon (14g), Tuna (14g), Avocado (7g), Mango (7g), MahachanokMango (7g), Pomelo (14g), Tamago (14g), Edamame (14g), Eel (14g), Ikura (14g), Others (259g)
▸ A1:AE33 table 33×31 (933 cells), header row 2
  A "รหัส": num 1001 … 3002 (26 filled)
  B "CSD": text 31 distinct e.g. "Avocado", "Edamame", "Eel" (31 filled)
  C "ราคาต่อกรัม": ƒ414,ƒ415,ƒ416 +13; num 0.02 … 180.00 fmt 0.00 (26 filled)
  D "คงเหลือ ⏎Raw material": ƒ1,ƒ8,ƒ15 +28; num 0 … 8540 (31 filled)
  E "คงเหลือ ⏎Ready to use": ƒ51,ƒ65,ƒ79 +20; num 0 … 1500 (23 filled)
  F "คงเหลือ⏎Total": ƒ379,ƒ407,ƒ386; num 0 … 8540 (31 filled)
  G "[31-Dec-2022] มูลค่า": ƒ400; num 0.00 … 8,160.00 fmt #,##0.00;(#,##0.00) (31 filled)
  H "คงเหลือ ⏎Raw material": ƒ2,ƒ9,ƒ16 +28; num 0 … 6500 (31 filled)
  I "คงเหลือ ⏎Ready to use": ƒ53,ƒ67,ƒ81 +20; num 0 … 1591 (23 filled)
  J "คงเหลือ⏎Total": ƒ380,ƒ408,ƒ387; num 0 … 6603 (31 filled)
  K "[31-Jan-2023] มูลค่า": ƒ401; num 0 … 4335 (31 filled)
  L "คงเหลือ ⏎Raw material": ƒ3,ƒ10,ƒ17 +28; num 0 … 8500 (31 filled)
  M "คงเหลือ ⏎Ready to use": ƒ55,ƒ69,ƒ83 +20; num 0 … 1813 (23 filled)
  N "คงเหลือ⏎Total": ƒ381,ƒ409,ƒ388; num 0 … 8654 (31 filled)
  O "[28-Feb-2023] มูลค่า": ƒ402; num 0 … 6033.75 (31 filled)
  P "คงเหลือ ⏎Raw material": ƒ4,ƒ11,ƒ18 +28; num 0 … 10410 (31 filled)
  Q "คงเหลือ ⏎Ready to use": ƒ57,ƒ71,ƒ85 +20; num 0 … 1496 (23 filled)
  R "คงเหลือ⏎Total": ƒ382,ƒ410,ƒ389; num 0 … 10410 (31 filled)
  S "[31-Mar-2023] มูลค่า": ƒ403; num 0 … 8850 (31 filled)
  T "คงเหลือ ⏎Raw material": ƒ5,ƒ12,ƒ19 +28; num 0 … 6731 (31 filled)
  U "คงเหลือ ⏎Ready to use": ƒ59,ƒ73,ƒ87 +20; num 0 … 1828 (23 filled)
  V "คงเหลือ⏎Total": ƒ383,ƒ411,ƒ390; num 0 … 6731 (31 filled)
  W "[30-Apr-2023] มูลค่า": ƒ404; num 0 … 12097.5 (31 filled)
  X "คงเหลือ ⏎Raw material": ƒ6,ƒ13,ƒ20 +28; num 0 … 8150 (31 filled)
  Y "คงเหลือ ⏎Ready to use": ƒ61,ƒ75,ƒ89 +20; num 0 … 1374 (23 filled)
  Z "คงเหลือ⏎Total": ƒ384,ƒ412,ƒ391; num 0 … 8150 (31 filled)
  AA "[31-May-2023] มูลค่า": ƒ405; num 0 … 6251.25 (31 filled)
  AB "คงเหลือ ⏎Raw material": ƒ7,ƒ14,ƒ21 +28; num 0 … 8610 (31 filled)
  AC "คงเหลือ ⏎Ready to use": ƒ63,ƒ77,ƒ91 +20; num 0 … 1988 (23 filled)
  AD "คงเหลือ⏎Total": ƒ385,ƒ413,ƒ392; num 0 … 8610 (31 filled)
  AE "[30-Jun-2023] มูลค่า": ƒ406; num 0 … 6041.25 (31 filled)
  sample (A…P of 31):
  3| 1002 | Salmon Proton | 0.62 | 8540 |  | 8540 | 5,294.80 | 3690 |  | 3690 | 2287.8 | 3210 |  | 3210 | 1990.2 | 10410
  4| 1001 | Salmon | 0.60 | 1440 |  | 1440 | 864.00 | 2400 |  | 2400 | 1440 | 1840 |  | 1840 | 1104 | 1312
  5| 1101 | Tuna (Frozen) | 0.50 | 1660 |  | 1660 | 830.00 | 0 |  | 0 | 0 | 0 |  | 0 | 0 | 2590
  33| 3002 | ข้าว | 180.00 | 18 |  | 18 | 3,240.00 | 4 |  | 4 | 720 | 4 |  | 4 | 720 | 21
▸ B35 (1 cells)
  35: B=Total
▸ G35 (1 cells)
  35: G=ƒ393→29,143.10
▸ K35 (1 cells)
  35: K=ƒ394→19,987.41
▸ O35 (1 cells)
  35: O=ƒ395→21,008.88
▸ S35 (1 cells)
  35: S=ƒ396→34,362.72
▸ W35 (1 cells)
  35: W=ƒ397→31,635.98
▸ AA35 (1 cells)
  35: AA=ƒ398→30,514.10
▸ AE35 (1 cells)
  35: AE=ƒ399→32,956.08
ƒ formulas (835 cells, 429 groups, 5 families)
ƒ1–378 378 similar groups (378 cells; same shape, refs/constants differ) e.g. D3  =INDEX(Salmon!$V:$V,match(G$1,Salmon!$A:$A,0),); others at H3, L3, P3, T3, X3, AB3 +371
ƒ379–399 21 similar groups (217 cells; same shape, refs/constants differ) e.g. F3:F7 ↓ =SUM(D3:E3); others at J3:J7, N3:N7, R3:R7, V3:V7, Z3:Z7, AD3:AD7 +14
ƒ400–406 7 similar groups (217 cells; same shape, refs/constants differ) e.g. G3:G33 ↓ =$F3*$C3; others at K3:K33, O3:O33, S3:S33, W3:W33, AA3:AA33, AE3:AE33
ƒ407–413 F8  =iferror(SUM(D8:E8),0) — same formula in 7 ranges (7 cells): J8, N8, R8, V8, Z8, AD8; breaks at G8:I8, K8:M8, O8:Q8, S8:U8, W8:Y8 +1 = 12 different formulas, 6 blank
typed arithmetic: 16 cells hold ref-free formulas (hand-entered sums) e.g. `=57.14/175` `=85.6/400` `=69.55/1000` in columns C
◆ formatting
bg#D9EAD3 (448 cells): D2:E33, H2:I33, L2:M33, P2:Q33 +3
bg#B6D7A8 (224 cells): F2:F33, J2:J33, N2:N33, R2:R33 +3
bg#93C47D (224 cells): G2:G33, K2:K33, O2:O33, S2:S33 +3
bg#CFE2F3 (19 cells): B15:B33
bold (8 cells): B35, G35, K35, O35 +4
bg#FCE5CD (7 cells): B3:B9
bg#FFF2CC (5 cells): B10:B14
bold underline (2 cells): A2:B2
✎ notes (2)
C17: ราคาต่อตัว
C33: ราคาต่อถุง

## 9. Inventory Summary · A1:I34 · 118 cells
reads: Fish record (6g), Salmon (3g), Tuna (2g), Avocado (3g), Mango (3g), MahachanokMango (1g), Pomelo (3g), Tamago (5g), Edamame (5g), Eel (2g), Ikura (2g), Others (42g)
▸ A1:D2 (5 cells)
  1: A=Date B=ƒ1→10/23/2026
  2: A=CSD C=คงเหลือ Raw materials  … D=คงเหลือ (Beginning Bala…
▸ G1:I5 (9 cells)
  1: G=ƒ2→10/22/2026
  2: G=Waste : Production H=Expired
  3: H=Production I=Ready to use
  4: H=ƒ4→
  5: G=ƒ6→190 H=ƒ7→0 I=ƒ8→105
▸ A4:D34 table 31×4 (85 cells), no header row
  A -: text 31 distinct e.g. "Avocado", "Edamame", "Eel" (31 filled)
  C -: ƒ3,ƒ5,ƒ9 +28; num 0 … 8880 (31 filled)
  D -: ƒ22,ƒ25,ƒ30 +20; num 0 … 1615 (23 filled)
  sample (A…D):
  4| Salmon Proton |  | 8060 | 
  5| Salmon |  | 2611 | 
  6| Tuna (Frozen) |  | 4910 | 
  34| ข้าว |  | 25 | 
▸ G7:I8 (5 cells)
  7: G=ƒ11→0 H=ƒ12→0 I=ƒ13→0
  8: G=ƒ15→255 H=ƒ16→206
▸ G10:H10 (2 cells)
  10: G=ƒ19→ H=ƒ20→362
▸ G11:I13 (7 cells)
  11: I=ƒ23→
  12: G=ƒ26→ H=ƒ27→ I=ƒ28→
  13: G=ƒ31→ H=ƒ32→ I=ƒ33→
▸ I20 (1 cells)
  20: I=ƒ48→
▸ G22 (1 cells)
  22: G=ƒ53→250
▸ I25 (1 cells)
  25: I=ƒ60→
▸ G26 (1 cells)
  26: G=ƒ63→
▸ I28 (1 cells)
  28: I=ƒ68→66
ƒ formulas (79 cells, 79 groups, 3 families)
ƒ1 B1 =today() [volatile]
ƒ2 G1 =today()-1 [volatile]
ƒ3–79 77 similar groups (77 cells; same shape, refs/constants differ) e.g. C4  =INDEX(Salmon!$V:$V,match($B$1,Salmon!$A:$A,0),); others at H4, C5, G5, H5, I5, C6 +70
◆ formatting
bg#FFF2CC (48 cells): C16:C34, D16:D33, G8:H8, G12:H12 +6
bg#D9EAD3 (5 cells): G10:H10, G13:I13
bg#FCE5CD (3 cells): G5:I5
bg#F4CCCC (3 cells): G7:I7
bg#FFFF00 (2 cells): B1, G1
bold underline (1 cells): A2
merged: A2:A3, C2:C3, D2:D3, E2:E3, G2:G3 +1

## 10. Fish record · A1:W1869 · 27k cells · frozen 2r1c
read by: Monthly Summary (Usage), Inventory Summary, Salmon, Tuna
▸ A1:W1869 table 1869×23 (27035 cells), header row 2
  A "[1] Date": date Tue 01-03-2022  … Wed 31-03-2027  (1857 filled)
  B "[2] Month": ƒ2; num 1 … 12 (1857 filled)
  C "[3] Year": ƒ3; num 2022 … 2027 (1857 filled)
  D "[4] Salmon Total Weight": num 0 … 7015 (1768 filled)
  E "[5] Salmon Waste": num 0 … 5315 (1768 filled)
  F "[6] Salmon Waste (Closed)": num 0 … 775 (1768 filled)
  G "[7] Salmon Waste (expired)": num -1946 … 0 (1768 filled)
  H "[8] Salmon Net Weight": ƒ5; num -346 … 4635 (1864 filled)
  I "[9] Salmon Yield": ƒ7; num -8.91% … 96.93% fmt 0.00%; 167 errors (1864 filled)
  J "[10] Tuna Total Weight": num 0 … 2653 (1768 filled)
  K "[11] Tuna Waste": num 0 … 358 (1768 filled)
  L "[12] Tuna Waste (Closed)": num 0 … 400 (1582 filled)
  M "[13] Tuna Waste (expired)": num 0 (1768 filled)
  N "[14] Tuna Net Weight": ƒ6; num 0 … 2653 (1866 filled)
  O "[15] Tuna Yield": ƒ8; num 71.13% … 100.00% fmt 0.00%; 173 errors (1867 filled)
  P "[16] TCM": empty
  Q "[17] $/g": empty
  R "[18] Discount": empty
  S "[19] Refund": empty
  T "[20] Non Poke Revenue": empty
  U "[21] $ excl. non poke": empty
  V "[22] $/g excl. non poke": empty
  W "Total Waste (closed)": empty
  sample (A…P of 23):
  3| Tue 01-03-2022  | 3 | 2022 | 3047 | 293 | 143 | 0 | 2754 | 90.38% | 1059 | 0 | 51 | 0 | 1059 | 100.00% | 
  4| Wed 02-03-2022  | 3 | 2022 | 4209 | 437 | 286 | 0 | 3772 | 89.62% | 1384 | 21 | 147 | 0 | 1363 | 98.48% | 
  5| Thu 03-03-2022  | 3 | 2022 | 3416 | 340 | 71 | 0 | 3076 | 90.05% | 790 | 0 | 19 | 0 | 790 | 100.00% | 
  1869|  |  |  |  |  |  |  |  |  |  |  |  |  |  | #DIV/0! Function DIVIDE… | 
ƒ formulas (11k cells, 9 groups, 7 families)
ƒ1 B1:V1 → =A1+1
ƒ2 B3:B1859 ↓ =month(A3)
ƒ3 C3:C1859 ↓ =Year(A3)
ƒ4 D3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","Fish record summary!D3:g2000") [external] [array/spills]
ƒ5–6 H3:H1866 ↓ =D3-E3 — same formula in 2 ranges (3730 cells): N3:N1868
ƒ7–8 I3:I1866 ↓ =H3/D3 [167 errors] — same formula in 2 ranges (3731 cells): O3:O1869
ƒ9 J3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","Fish record summary!j3:m2000") [external] [array/spills]
◆ formatting
fg#0000FF (9318 cells): A3:A1859, H3:I1866, N3:N1868, O3:O1869
italic (22 cells): A1:V1
bg#D9D9D9 bold underline (7 cells): P2:V2
bg#FCE5CD bold underline (6 cells): D2:I2
bg#F4CCCC bold underline (6 cells): J2:O2
bold underline (4 cells): A2:C2, W2
bg#FCE5CD (2 cells): D3, J3

## 11. Salmon · A1:V3001 · 52k cells · frozen 3r3c
reads: Sales (1g), Item Sales (Usage) (2g), Fish record (7g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
▸ A1:V3001 table 3001×22 (51794 cells), header row 3
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Wed, May 15, 2030 (2998 filled)
  B "[1] Month": ƒ4; num 1 … 12 (2998 filled)
  C "[2] Year": ƒ5; num 2022 … 2030 (2998 filled)
  D "[Salmon (กรัม)] Salmon (กรัม/1000บาท)": ƒ6; num 40.58 … 394.70 fmt 0.00; 1301 errors (2998 filled)
  E "[3 / 1/1] คงเหลือ": num 0 … 14761 (1698 filled)
  F "[4] ของเข้า(+)": num 848 … 13750 (1363 filled)
  G "[5] ของออก(-)": num 1049 … 4190 (12 filled)
  H "[6] ไม่ได้คุณภาพ(-)": num 32 … 220 (8 filled)
  I "[7] หมดอายุ(-)": I67=633 I141=1946
  J "[8] ใช้ไป (ใบของสด)": num 0 … 7330 (1998 filled)
  K "[9] ใช้ไป(ใบจดปลา) ก่อน waste": ƒ8,ƒ15; num 0 … 7015 (2998 filled)
  L "[10] (ใบนับของสด-ใบจดปลา)": ƒ17; num -322 … 2611 (2998 filled)
  M "[11] % diff": ƒ18; num -13.64% … 35.95% fmt 0.00%; 1301 errors (2998 filled)
  N "[12] Check ใบของสด": ƒ20; enum Correct(1586) Re-check(111); 1301 errors (2998 filled)
  O "[13] ใช้ไป(ใบจดปลา) หลัง waste": ƒ9; num -346 … 4635 (2998 filled)
  P "[14] waste ปิดร้าน (-)": ƒ10,ƒ14; num 0 … 775 (2998 filled)
  Q "[15] waste หมดอายุ (-)": ƒ11,ƒ13; num -1946 … 0 (2998 filled)
  R "[16] item sales": ƒ12,ƒ16; num 0 … 4140 (2998 filled)
  S "[17] ใบจดปลา-Item sales": ƒ22; num -397 … 461 fmt 0 (2998 filled)
  T "[18] % diff": ƒ19; num -20.31% … 21.87% fmt 0.00%; 1301 errors (2998 filled)
  U "[19] Check item sales": ƒ21; enum Correct(1670) Re-check(27); 1301 errors (2998 filled)
  V "[20] คงเหลือ Proton": num 0 … 17670 (1698 filled)
  sample (A…P of 22):
  4| Tue, Mar 1, 2022 | 3 | 2022 | 124.12 | 4410 |  |  |  |  | 3240 | 3047 | 193 | 6.33% | Correct | 2754 | 143
  5| Wed, Mar 2, 2022 | 3 | 2022 | 137.65 | 1170 | 6370 |  |  |  | 4440 | 4209 | 231 | 5.49% | Correct | 3772 | 286
  6| Thu, Mar 3, 2022 | 3 | 2022 | 154.77 | 3100 | 3140 |  |  |  | 3700 | 3416 | 284 | 8.31% | Correct | 3076 | 71
  3001| Wed, May 15, 2030 | 5 | 2030 | #DIV/0! Function DIVIDE… |  |  |  |  |  |  |  | 0 | #DIV/0! Function DIVIDE… | #DIV/0! Function DIVIDE… |  | 
ƒ formulas (42k cells, 23 groups, 11 families)
ƒ1–3 3 similar groups (20 cells; same shape, refs/constants differ) e.g. B1:C1 → =A1+1; others at E1, F1:V1
ƒ4 B4:B3001 ↓ =month(A4)
ƒ5 C4:C3001 ↓ =year(A4)
ƒ6 D4:D3001 ↓ =K4/Sales!$J4*1000 [1301 errors]
ƒ7 E4 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","1/1!c3:h2000") [external] [array/spills]
ƒ8–16 9 similar groups (14990 cells; same shape, refs/constants differ) e.g. K4:K1973 ↓ ='Fish record'!D3; others at O4:O3001, P4:P1719, Q4:Q1697, R4:R2000, Q1698:Q3001, P1720:P3001 +2
ƒ17 L4:L3001 ↓ =J4-K4
ƒ18–19 M4:M3001 ↓ =L4/K4 [1301 errors] — same formula in 2 ranges (5996 cells): T4:T3001; breaks at N4:S3001 = 17988 different formulas
ƒ20 N4:N3001 ↓ =if(and(M4<10%,M4>-10%),"Correct","Re-check") [1301 errors]
ƒ21 U4:U3001 ↓ =if(and(T4<13%,T4>-5%),"Correct","Re-check") [1301 errors]
ƒ22 S4:S3001 ↓ =O4-P4-Q4-R4
ƒ23 V4 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","1/1!k3:k2000") [external] [array/spills]
◐ conditional formats
N2:N3001, U4:U3001: TEXT_EQ Correct → bg#B7E1CD
N2:N3001, U4:U3001: TEXT_EQ Re-check → bg#EA9999
A1:A3001: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#EFEFEF (26982 cells): L4:Q3001, S4:U3001
bg#FCE5CD (26981 cells): D4:D3001, E5:E3001, F4:K3001, V2 +1
bg#D9EAD3 fg#0000FF (2998 cells): R4:R3001
italic (22 cells): A1:V1
bg#EFEFEF bold (18 cells): L2:Q3, S2:U3
bg#FCE5CD bold (17 cells): D2:K3, V3
bold (3 cells): A2:C2
bold underline (3 cells): A3:C3
+3 more styles
hidden rows: 4-308, 310-673, 676-1405

## 12. Tuna · A1:V2000 · 37k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (2g), Fish record (7g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Salmon" (81% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Tuna"`
blocks: A1:V2000 (36873 cells)
ƒ 28k cells in 22 groups; 2 families not in "Salmon":
ƒ21 V3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","1/2!c3:c2000") [external] [array/spills]
ƒ22 E12 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","1/1!s12:x2000") [external] [array/spills]
labels not in "Salmon": D:"Tuna (กรัม)" D:"Tuna (กรัม/1000บาท)" V:"คงเหลือ Tuna (Frozen)"

## 13. Chicken · A1:V1570 · 24k cells · frozen 2r3c
reads: Sales (1g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage)
▸ A1:V1570 table 1570×22 (24446 cells), header row 2
  A "Date": date Thu, Jun 1, 2023 … Wed, Sep 15, 2027 (1568 filled)
  B "Month": ƒ1; num 1 … 12 (1568 filled)
  C "Year": ƒ2; num 2023 … 2027 (1568 filled)
  D "[Chicken (กรัม)] Chicken (กรัม/1000บาท)": ƒ3; num 0.00 … 677.38 fmt 0.00 (1568 filled)
  E "[9/2] คงเหลือ": num 0 … 19000 (1241 filled)
  F "ของเข้า(+)": num 400 … 12000 (532 filled)
  G "ของออก(-)": G724=3700 G1072=1000
  H "ไม่ได้คุณภาพ(-)": num 0 … 900 (176 filled)
  I "หมดอายุ(-)": empty
  J "ใช้ไป (ใบของสด)": num 0 … 5000 (1541 filled)
  K "เตรียมได้": num 100 … 5207 (872 filled)
  L "[5/3] ใช้ไป⏎(ใบเตรียมของ)": num 0 … 4408 (1541 filled)
  M "ไม่ได้คุณภาพ(-)": ƒ7 (1 filled)
  N "หมดอายุ(-)": num 40 … 1066 (25 filled)
  O "เปิดร้าน (ใบเตรียมของ)": num 0 … 4259 (1241 filled)
  P "(ใบนับของสด-เตรียมได้)": ƒ10; num (400) … 150  fmt #,##0_);[Red](#,##0) (1568 filled)
  Q "[95-115%] % diff": ƒ12; num 0.00% … 142.26% fmt 0.00% (1568 filled)
  R "Check ใบของสด": ƒ13; enum Correct(1493) Re-check(75) (1568 filled)
  S "item sales": ƒ14,ƒ15; num 0 … 4270 (1568 filled)
  T "ใบเตรียมของ-Item sales": ƒ11; num (226) … 1,000  fmt #,##0_);[Red](#,##0) (1568 filled)
  U "% diff": ƒ16; num -29.17% … 83.33% fmt 0.00%; 336 errors (1568 filled)
  V "Check item sales": ƒ17; enum Correct(1145) Re-check(87); 336 errors (1568 filled)
  sample (A…P of 22):
  3| Thu, Jun 1, 2023 | 6 | 2023 | 53.79 |  | 8000 |  |  |  | 1000 | 1000 | 800 |  | 100 |  | 0 
  4| Fri, Jun 2, 2023 | 6 | 2023 | 13.33 | 7000 |  |  |  |  | 200 | 200 | 200 |  |  | 100 | 0 
  5| Sat, Jun 3, 2023 | 6 | 2023 | 20.73 | 6800 |  |  |  |  | 300 | 300 | 300 |  | 100 | 100 | 0 
  1570| Wed, Sep 15, 2027 | 9 | 2027 | 0.00 |  |  |  |  |  |  |  |  |  |  |  | 0 
ƒ formulas (16k cells, 17 groups, 15 families)
ƒ1 B3:B1570 ↓ =month(A3)
ƒ2 C3:C1570 ↓ =year(A3)
ƒ3 D3:D1570 ↓ =L3/Sales!$G4*1000
ƒ4 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1674035320","9/2!K460:P2000") [external] [array/spills]
ƒ5 K3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=2006911781","5/3!E460:E2000") [external] [array/spills]
ƒ6 L3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=2006911781","5/3!J460:J2000") [external] [array/spills]
ƒ7 M3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=2006911781","5/3!G460:G2000") [external] [array/spills]
ƒ8 N3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=2006911781","5/3!H460:H2000") [external] [array/spills]
ƒ9 O3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=2006911781","5/3!C460:C2000") [external] [array/spills]
ƒ10 P3:P1570 ↓ =J3-K3
ƒ11 T3:T1570 ↓ =L3-S3
ƒ12 Q3:Q1570 ↓ =iferror(K3/J3,0)
ƒ13 R3:R1570 ↓ =if(Or(and(Q3<115%,Q3>95%),and(P3=0,Q3=0%)),"Correct","Re-check")
ƒ14 S3:S1542 ↓ ='Item Sales (Usage)'!L461
ƒ15 S1543:S1570 ↓ ='Item Sales (Usage)'!L3001
ƒ16 U3:U1570 ↓ =T3/S3 [336 errors]
ƒ17 V3:V1570 ↓ =if(and(U3<13%,U3>-5%),"Correct","Re-check") [336 errors]
◐ conditional formats
R1:R2570, V3:V2570: TEXT_EQ Correct → bg#B7E1CD
R1:R2570, V3:V2570: TEXT_EQ Re-check → bg#EA9999
A1:A2570: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (18810 cells): D3:D1570, E4:E1570, F3:J1570, K4:O1570
bg#EFEFEF (9408 cells): P3:R1570, T3:V1570
bg#D9EAD3 fg#0000FF (1568 cells): S3:S1570
bg#FCE5CD bold (24 cells): D1:O2
bg#EFEFEF bold (12 cells): P1:R2, T1:V2
bg#F9CB9C (6 cells): E3, K3:O3
bold (3 cells): A1:C1
bold underline (3 cells): A2:C2
+2 more styles
hidden rows: 4-215, 218-947

## 14. ChickenBB · A1:V2500 · 32k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage)
same layout as "Chicken" (86% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "ChickenBB"`
blocks: A1:V2500 (32376 cells)
ƒ 25k cells in 18 groups; 6 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1674035320","10/3!C3:H2000") [external] [array/spills]
ƒ6 K3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=2006911781","5/3!N460:N2000") [external] [array/spills]
ƒ7 L3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=2006911781","5/3!S460:S2000") [external] [array/spills]
ƒ8 M3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=2006911781","5/3!P460:P2000") [external] [array/spills]
ƒ9 N3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=2006911781","5/3!Q460:Q2000") [external] [array/spills]
ƒ10 O3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=2006911781","5/3!L460:L2000") [external] [array/spills]
labels not in "Chicken": A:"รายการ" D:"Chicken BB (กรัม)" D:"Chicken BB (กรัม/1000บา…"

## 15. Avocado · A1:N2000 · 23k cells · frozen 2r0c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
▸ A1:N2000 table 2000×14 (22684 cells), header row 2
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Thu, Aug 19, 2027 (1998 filled)
  B "Month": ƒ1; num 1 … 12 (1998 filled)
  C "Year": ƒ2; num 2022 … 2027 (1998 filled)
  D "[Avocado (กรัม)] อโวคาโด (กรัม/1000บาท)": ƒ3,ƒ4; num 25.07 … 204.06 fmt 0.00; 301 errors (1998 filled)
  E "[2/1] คงเหลือ": num 0 … 13020 (1698 filled)
  F "ของเข้า(+)": num 1172 … 11680 (802 filled)
  G "ของออก(-)": num 335 … 1330 (3 filled)
  H "ไม่ได้คุณภาพ(-)": num 0 … 6955 (975 filled)
  I "หมดอายุ(-)": num 0 … 716 (1206 filled)
  J "ใช้ไป (ใบของสด)": num 0 … 7865 (1998 filled)
  K "item sales": ƒ6,ƒ7; num 0 … 2004 (1998 filled)
  L "ใช่ไปใบของสด-Item sales": ƒ8; num -59 … 7865 (1998 filled)
  M "[40%] % Yield": ƒ9; num 0.00% … 109.09% fmt 0.00%; 300 errors (1998 filled)
  N "Check item sales": ƒ10; enum Correct(963) Re-check(735); 300 errors (1998 filled)
  sample (A…N):
  3| Tue, Mar 1, 2022 | 3 | 2022 | 80.25 | 5150 | 4900 |  |  |  | 1970 | 684 | 1286 | 34.72% | Re-check
  4| Wed, Mar 2, 2022 | 3 | 2022 | 66.98 | 8080 |  |  | 163 | 109 | 2048 | 768 | 1280 | 37.50% | Re-check
  5| Thu, Mar 3, 2022 | 3 | 2022 | 64.75 | 5760 | 2680 |  |  | 113 | 1429 | 576 | 853 | 40.31% | Correct
  2000| Thu, Aug 19, 2027 | 8 | 2027 | #DIV/0! Function DIVIDE… |  |  |  |  |  | 0 |  | 0 | #DIV/0! Function DIVIDE… | #DIV/0! Function DIVIDE…
ƒ formulas (14k cells, 10 groups, 8 families)
ƒ1 B3:B2000 ↓ =month(A3)
ƒ2 C3:C2000 ↓ =year(A3)
ƒ3 D3:D1977 ↓ =J3/Sales!$J4*1000 [278 errors]
ƒ4 D1978:D2000 ↓ =J1978/Sales!$J4383*1000 [23 errors]
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","2/1!c3:h2000") [external] [array/spills]
ƒ6 K3:K1999 ↓ ='Item Sales (Usage)'!G4
ƒ7 K2000 ='Item Sales (Usage)'!G3001
ƒ8 L3:L2000 ↓ =J3-K3
ƒ9 M3:M2000 ↓ =K3/J3 [300 errors]
ƒ10 N3:N2000 ↓ =if(and(M3<100%,M3>40%),"Correct","Re-check") [300 errors]
◐ conditional formats
N3:N3000: TEXT_EQ Correct → bg#B7E1CD
N3:N3000: TEXT_EQ Re-check → bg#EA9999
A1:A3000: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (13985 cells): D3:D2000, E4:E2000, F3:J2000
bg#EFEFEF (9991 cells): A3:C2000, L3:M2000, N1
bg#D9EAD3 fg#0000FF (1998 cells): K3:K2000
bg#B7E1CD (1998 cells): N3:N2000
bg#FCE5CD bold (14 cells): D1:J2
bg#EFEFEF bold (8 cells): A1:C1, L1:M2, N2
bg#EFEFEF bold underline (3 cells): A2:C2
bg#D9EAD3 bold (2 cells): K1:K2
+1 more styles
hidden rows: 4-307, 309-672, 675-1404

## 16. Mango · A1:N2000 · 22k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (3g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Avocado" (77% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Mango"`
blocks: A1:N2000 (21988 cells)
ƒ 14k cells in 11 groups; 1 families not in "Avocado":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","1/2!k3:p2000") [external] [array/spills]
labels not in "Avocado": D:"Mango (กรัม)" D:"มะม่วง (กรัม/1000บาท)" N:"Check Yield"
✎ J1530: รวมเม็ด ยังไม่ปลอก⏎

## 17. MahachanokMango · A1:N2000 · 20k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Avocado" (77% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "MahachanokMango"`
blocks: A1:N2000 (19983 cells)
ƒ 14k cells in 10 groups; 1 families not in "Avocado":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","1/2!s3:x2000") [external] [array/spills]
labels not in "Avocado": D:"Mahachanok Mango (กรัม)" D:"มะม่วง (กรัม/1000บาท)" M:"% diff"

## 18. Pomelo · A1:V2000 · 32k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Chicken" (69% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Pomelo"`
blocks: A1:V2000 (31935 cells)
ƒ 20k cells in 17 groups; 5 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","6/2!c3:h2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","3/1!n3:n2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","3/1!s3:s2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","3/1!p3:q2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","3/1!l3:l2000") [external] [array/spills]
labels not in "Chicken": D:"Pomelo (กรัม)" D:"ส้มโอ (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-เตรียมได้" Q:"% Yield"

## 19. Tamago · A1:V2000 · 31k cells · frozen 2r0c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Chicken" (62% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Tamago"`
blocks: A1:V2000 (31264 cells)
ƒ 20k cells in 17 groups; 6 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","3/2!k3:p2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","2/1!n3:n2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","2/1!s3:s2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","2/1!p3:q2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","2/1!l3:l2000") [external] [array/spills]
ƒ17 V3:V2000 ↓ =if(and(U3<=20%,U3>-5%),"Correct","Re-check") [301 errors]
labels not in "Chicken": A:"รายการ" D:"Tamago (กรัม)" D:"ไข่หวาน (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-ใบเตรียมของ" Q:"% Yield" V:"-5%-20%"

## 20. Edamame · A1:V2000 · 31k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Chicken" (62% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Edamame"`
blocks: A1:V2000 (31192 cells)
ƒ 20k cells in 17 groups; 6 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","3/1!s3:x2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!w3:w2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!ab3:ab2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!y3:z2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!u3:u2000") [external] [array/spills]
ƒ17 V3:V2000 ↓ =if(and(U3<=20%,U3>-5%),"Correct","Re-check") [301 errors]
labels not in "Chicken": A:"รายการ" D:"Edamame (กรัม)" D:"ถั่วแระ (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-เตรียมได้" Q:"% Yield" V:"-5%-20%"

## 21. Eel · A1:R2000 · 26k cells · frozen 2r3c
reads: Sales (2g)
read by: Monthly Inventory Summary, Inventory Summary
▸ A1:R2000 table 2000×18 (25797 cells), header row 2
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Thu, Aug 19, 2027 (1998 filled)
  B "Month": ƒ1; num 1 … 12 (1998 filled)
  C "Year": ƒ2; num 2022 … 2027 (1998 filled)
  D "[ปลาไหล ] ปลาไหล (กรัม/1000บาท)": ƒ3,ƒ4; num 0.00 … 35.53 fmt 0.00; 301 errors (1998 filled)
  E "[2/2] คงเหลือ": num 446 … 7359 (1698 filled)
  F "ของเข้า(+)": num 1784 … 4693 (64 filled)
  G "ของออก(-)": num 446 … 2230 (4 filled)
  H "ไม่ได้คุณภาพ(-)": empty
  I "หมดอายุ(-)": empty
  J "ใช้ไป (ใบของสด)": num 0 … 4683 (1998 filled)
  K "[1/1] เตรียมได้": num 175 … 642 (1079 filled)
  L "ใช้ไป(ใบเตรียมของ) 5g": num 0 … 532 (1998 filled)
  M "เปิดร้าน (ใบเตรียมของ)": num 0 … 442; text "-" (1698 filled)
  N "(ใบนับของสด-เตรียมได้": ƒ9; num -214 … 4683 (1998 filled)
  O "[90%] % Yield": ƒ11; num 0% … 913% fmt 0% (1998 filled)
  P "Check ใบของสด": ƒ12; enum Correct(1894) Re-check(104) (1998 filled)
  Q "[5/2] เตรียมได้(ใบเตรียมของ) 2.5g": num 9 … 532 (1250 filled)
  R "⏎ปลาไหลเตรียมได้ (5g)        …": ƒ10; num -43 … 297 (1998 filled)
  sample (A…P of 18):
  3| Tue, Mar 1, 2022 | 3 | 2022 | 9.08 | 2230 |  |  |  |  | 223 | 208 | 154 | 154 | 15 | 93% | Correct
  4| Wed, Mar 2, 2022 | 3 | 2022 | 0.00 | 2007 |  |  |  |  | 0 |  | 208 | 208 | 0 | 0% | Correct
  5| Thu, Mar 3, 2022 | 3 | 2022 | 20.21 | 2007 |  |  |  |  | 446 |  | 258 | - | 446 | 0% | Re-check
  2000| Thu, Aug 19, 2027 | 8 | 2027 | #DIV/0! Function DIVIDE… |  |  |  |  |  | 0 |  | 0 |  | 0 | 0% | Correct
ƒ formulas (14k cells, 13 groups, 11 families)
ƒ1 B3:B2000 ↓ =month(A3)
ƒ2 C3:C2000 ↓ =year(A3)
ƒ3 D3:D1999 ↓ =J3/Sales!$J4*1000 [300 errors]
ƒ4 D2000 =J2000/Sales!$J3405*1000 [1 errors]
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","2/2!c3:h2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/1!e3:e2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/1!j3:j2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/1!c3:c2000") [external] [array/spills]
ƒ9 N3:N2000 ↓ =J3-K3
ƒ10 R3:R2000 ↓ =L3-Q3
ƒ11 O3:O2000 ↓ =iferror(K3/J3,0)
ƒ12 P3:P2000 ↓ =if(Or(and(O3<100%,O3>=90%),and(N3=0,O3=0%)),"Correct","Re-check")
ƒ13 Q3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","5/2!n3:n2000") [external] [array/spills]
◐ conditional formats
P1:P2000, T3:T4: TEXT_EQ Correct → bg#B7E1CD
P1:P2000, T3:T4: TEXT_EQ Re-check → bg#EA9999
A1:A2000: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (21973 cells): D3:D2000, E4:E2000, F3:J2000, K4:M2000 +1
bg#EFEFEF (11988 cells): A3:C2000, N3:O2000, R3:R2000
bg#B7E1CD (1998 cells): P3:P2000
bg#FCE5CD bold (22 cells): D1:M2, Q1:Q2
bg#EFEFEF bold (11 cells): A1:C1, N1:P2, R1:R2
bg#F9CB9C (5 cells): E3, K3:M3, Q3
bg#EFEFEF bold underline (3 cells): A2:C2
hidden rows: 4-307, 309-672, 674-703, 705-732, 735-1404
hidden cols: F-I

## 22. Ikura · A1:V2000 · 30k cells · frozen 2r0c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
▸ A1:V2000 table 2000×22 (30166 cells), header row 2
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Thu, Aug 19, 2027 (1998 filled)
  B "Month": ƒ1; num 1 … 12 (1998 filled)
  C "Year": ƒ2; num 2022 … 2027 (1998 filled)
  D "[Ikura (กรัม)] ไข่ปลา (กรัม/1000บาท)": ƒ3,ƒ4; num 0.00  … 12.71  fmt #,##0.00_);[Red](#,##0.00); 301 errors (1998 filled)
  E "คงเหลือ": num 0 … 5000 (1698 filled)
  F "ของเข้า(+)": num 500 … 3000 (234 filled)
  G "ของออก(-)": G318=500
  H "ไม่ได้คุณภาพ(-)": num 1 … 16 (36 filled)
  I "หมดอายุ(-)": empty
  J "ใช้ไป (ใบของสด)": num 0 … 1000 (1998 filled)
  K "เตรียมได้": num 250 … 514 (388 filled)
  L "ใช้ไป(ใบเตรียมของ)": num 0 … 333 (1998 filled)
  M "ไม่ได้คุณภาพ(-)": ƒ8 (1 filled)
  N "หมดอายุ(-)": num 5 … 210 (111 filled)
  O "เปิดร้าน (ใบเตรียมของ)": num 0 … 644 (1698 filled)
  P "(ใบนับของสด-ใบเตรียมของ": ƒ10; num -500 … 1000 (1998 filled)
  Q "[100%] % Yield": ƒ12; num 0% … 103% fmt 0% (1998 filled)
  R "Check ใบของสด": ƒ13; enum Correct(1989) Re-check(9) (1998 filled)
  S "item sales": ƒ14,ƒ15; num 0 … 312 (1998 filled)
  T "ใบเตรียมของ-Item sales": ƒ11; num (33.00) … 49.00  fmt #,##0.00_);[Red](#,##0.00) (1998 filled)
  U "% diff": ƒ16; num -26.39% … 100.00% fmt 0.00%; 302 errors (1998 filled)
  V "Check item sales": ƒ17; enum Correct(1480) Re-check(216); 302 errors (1998 filled)
  sample (A…P of 22):
  3| Tue, Mar 1, 2022 | 3 | 2022 | 8.47  | 3000 |  |  |  |  | 500 | 500 | 208 |  |  | 206 | 0
  4| Wed, Mar 2, 2022 | 3 | 2022 | 6.87  | 2500 |  |  |  |  | 0 |  | 210 |  |  | 498 | 0
  5| Thu, Mar 3, 2022 | 3 | 2022 | 7.57  | 2500 |  |  |  |  | 0 |  | 167 |  |  | 288 | 0
  2000| Thu, Aug 19, 2027 | 8 | 2027 | #DIV/0! Function DIVIDE… |  |  |  |  |  | 0 |  | 0 |  |  |  | 0
ƒ formulas (20k cells, 17 groups, 14 families)
ƒ1 B3:B2000 ↓ =month(A3)
ƒ2 C3:C2000 ↓ =year(A3)
ƒ3 D3:D1999 ↓ =L3/Sales!$J4*1000 [300 errors]
ƒ4 D2000 =L2000/Sales!$J3405*1000 [1 errors]
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","3/1!k3:p2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!n3:n2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!s3:s2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!p3:q2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!l3:l2000") [external] [array/spills]
ƒ10 P3:P2000 ↓ =J3-K3
ƒ11 T3:T2000 ↓ =L3-S3
ƒ12 Q3:Q2000 ↓ =iferror(K3/J3,0)
ƒ13 R3:R2000 ↓ =if(Or(and(Q3<=100%,Q3>90%),and(P3=0,Q3=0%)),"Correct","Re-check")
ƒ14 S3:S1999 ↓ ='Item Sales (Usage)'!K4
ƒ15 S2000 ='Item Sales (Usage)'!K3001
ƒ16 U3:U2000 ↓ =T3/S3 [302 errors]
ƒ17 V3:V2000 ↓ =if(and(U3<=20%,U3>-5%),"Correct","Re-check") [302 errors]
◐ conditional formats
R1:R2000, V3:V2000: TEXT_EQ Correct → bg#B7E1CD
R1:R2000, V3:V2000: TEXT_EQ Re-check → bg#EA9999
A1:A2000: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (23971 cells): D3:D2000, E4:E2000, F3:J2000, K4:M2000 +2
bg#EFEFEF (15984 cells): A3:C2000, P3:Q2000, T3:V2000
bg#B7E1CD (1998 cells): R3:R2000
bg#D9EAD3 fg#0000FF (1998 cells): S3:S2000
bg#FCE5CD bold (24 cells): D1:O2
bg#EFEFEF bold (15 cells): A1:C1, P1:R2, T1:V2
bg#F9CB9C (5 cells): E3, K3:M3, O3
bg#EFEFEF bold underline (3 cells): A2:C2
+1 more styles
hidden rows: 4-307, 310-673, 675-1404

## 23. Others · A1:HW2681 · 311k cells · frozen 2r3c
reads: Sales (38g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
▸ A1:HW2681 table 2681×231 (311429 cells), header row 2
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Sat, Jun 30, 2029 (2679 filled)
  B "Month": ƒ1; num 1 … 12 (2679 filled)
  C "Year": ƒ2; num 2022 … 2029 (2679 filled)
  D "[Engawa] ครีบปลา (กรัม/1000บาท)": ƒ3,ƒ37; num 0.00 … 9.89 fmt 0.00; 982 errors (2679 filled)
  E "[2/2] คงเหลือ": num 0 … 1880 (1698 filled)
  F "ของเข้า(+)": num 244 … 1000 (132 filled)
  G "ของออก(-)": G765=632 G1651=370
  H "ไม่ได้คุณภาพ(-)": num 8 … 175 (42 filled)
  I "หมดอายุ(-)": I1482=10
  J "ใช้ไป (ใบของสด)": num -125 … 820 (1998 filled)
  K "เปิดร้าน (ใบเตรียมของ)": num 0 … 210 (1698 filled)
  L "[1/1] ใช้ไป(ใบเตรียมของ) ": num 0 … 230 (1998 filled)
  M "เตรียมได้": num 40 … 300 (619 filled)
  N "[100%] % Yield": ƒ57; num 0% … 137% fmt 0% (2679 filled)
  O "[Engawa] Check ใบของสด": ƒ76; enum Correct(2640) Re-check(39) (2679 filled)
  P "[ก้ามปู] ก้ามปู (กรัม/1000บาท)": ƒ4,ƒ29; num -0.69 … 11.85 fmt 0.00; 982 errors (2679 filled)
  Q "[2/2] คงเหลือ": num 200 … 2050 (1698 filled)
  R "ของเข้า(+)": num 100 … 1320 (146 filled)
  S "ของออก(-)": empty
  T "ไม่ได้คุณภาพ(-)": T765=80 T868=13
  U "หมดอายุ(-)": empty
  V "ใช้ไป (ใบของสด)": num 0 … 600 (1998 filled)
  W "เปิดร้าน (ใบเตรียมของ)": num 0 … 278 (1698 filled)
  X "[1/1] ใช้ไป(ใบเตรียมของ) ": num -19 … 280 (1998 filled)
  Y "เตรียมได้": num 85 … 300 (623 filled)
  Z "[100%] % Yield": ƒ58; num 0% … 100% fmt 0% (2679 filled)
  AA "[ก้ามปู] Check ใบของสด": ƒ77; enum Correct(2677) Re-check(2) (2679 filled)
  AB "[กุ้ง] กุ้ง (กรัม/1000บาท)": ƒ5,ƒ22; num -0.33 … 1.63 fmt 0.00; 982 errors (2679 filled)
  AC "[3/1] คงเหลือ": num 15 … 165 (1698 filled)
  AD "ของเข้า(+)": num 15 … 90 (280 filled)
  AE "ของออก(-)": empty
  AF "ไม่ได้คุณภาพ(-)": AF1515=4 AF1604=1
  AG "หมดอายุ(-)": empty
  AH "ใช้ไป (ใบของสด)": num 0 … 70 (1998 filled)
  AI "เปิดร้าน (ใบเตรียมของ)": num 0 … 22 (1697 filled)
  AJ "[1/2] ใช้ไป(ใบเตรียมของ) ": num -9 … 28 (1998 filled)
  AK "เตรียมได้": num 6 … 30 (779 filled)
  AL "[100%] % Yield": ƒ59; num 0% … 100% fmt 0% (2679 filled)
  AM "[กุ้ง] Check ใบของสด": ƒ78; enum Correct(2677) Re-check(2) (2679 filled)
  AN "[ยำสาหร่าย] ยำสาหร่าย (กรัม/1000บาท)": ƒ6,ƒ26; num 0.00 … 29.34 fmt 0.00; 982 errors (2679 filled)
  +191 more columns
  sample (A…P of 231):
  3| Tue, Mar 1, 2022 | 3 | 2022 | 0.00 | 1375 |  |  |  |  | 121 | 31 | 0 | 121 | 100% | Correct | 0.00
  4| Wed, Mar 2, 2022 | 3 | 2022 | 1.28 | 1254 |  |  |  |  | 0 | 152 | 39 |  | 0% | Correct | 2.52
  5| Thu, Mar 3, 2022 | 3 | 2022 | 0.68 | 1254 |  |  |  |  | 0 | 113 | 15 |  | 0% | Correct | 1.40
  2681| Sat, Jun 30, 2029 | 6 | 2029 | #DIV/0! Function DIVIDE… |  |  |  |  |  |  |  |  |  | 0% | Correct | #DIV/0! Function DIVIDE…
ƒ formulas (153k cells, 159 groups, 24 families)
ƒ1 B3:B2681 ↓ =month(A3)
ƒ2 C3:C2681 ↓ =year(A3)
ƒ3–40 38 similar groups (50900 cells; same shape, refs/constants differ) e.g. D3:D1734 ↓ =L3/Sales!$J4*1000 [35 errors]; others at P3:P1709, AB3:AB1699, AN3:AN1708, AZ3:AZ1711, BL3:BL1707, BX3:BX1703 +31
ƒ41 E3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","2/2!K3:P2000") [external] [array/spills]
ƒ42–46 5 similar groups (5 cells; same shape, refs/constants differ) e.g. K3  =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/1!l3:l2000") [external] [array/spills]; others at CE3, EA3, FK3, GI3
ƒ47–51 5 similar groups (5 cells; same shape, refs/constants differ) e.g. L3  =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/1!s3:s2000") [external] [array/spills]; others at CF3, EB3, FL3, GJ3
ƒ52–56 5 similar groups (5 cells; same shape, refs/constants differ) e.g. M3  =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/1!n3:n2000") [external] [array/spills]; others at CG3, EC3, FM3, GK3
ƒ57–75 N3:N2681 ↓ =IFERROR(M3/(J3+H3),0) — same formula in 19 ranges (48305 cells): Z3:Z2681, AL3:AL2681, AX3:AX2681, BJ3:BJ2681, BV3:BV2681, CH3:CH2681 +12; breaks at O3:Y2681, AA3:AK2681, AM3:AW2681, AY3:BI2681, BK3:BU2681 +11 = 141732 hardcoded values, 93838 different formulas, 268082 blank
ƒ76–94 19 similar groups (48305 cells; same shape, refs/constants differ) e.g. O3:O2681 ↓ =if(Or(and(N3<=100%,N3>95%),and(M3=0,N3=0%)),"Correct","Re-check"); others at AA3:AA2681, AM3:AM2681, AY3:AY2681, BK3:BK2681, BW3:BW2681, CI3:CI2681 +12
ƒ95–101 7 similar groups (7 cells; same shape, refs/constants differ) e.g. Q3  =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","2/2!s3:x2000") [external] [array/spills]; others at BY3, CW3, EG3, FQ3, GO3, HA3
ƒ102–109 8 similar groups (8 cells; same shape, refs/constants differ) e.g. W3  =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/1!u3:u2000") [external] [array/spills]; others at BG3, CQ3, DC3, EM3, FW3, GU3 +1
ƒ110–117 8 similar groups (8 cells; same shape, refs/constants differ) e.g. X3  =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/1!ab3:ab2000") [external] [array/spills]; others at BH3, CR3, DD3, EN3, FX3, GV3 +1
ƒ118–125 8 similar groups (8 cells; same shape, refs/constants differ) e.g. Y3  =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/1!w3:w2000") [external] [array/spills]; others at BI3, CS3, DE3, EO3, FY3, GW3 +1
ƒ126–128 3 similar groups (3 cells; same shape, refs/constants differ) e.g. AC3  =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","3/1!c3:h2000") [external] [array/spills]; others at BM3, CK3
ƒ129–133 5 similar groups (5 cells; same shape, refs/constants differ) e.g. AI3  =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!c3:c2000") [external] [array/spills]; others at AU3, BS3, DO3, EY3
ƒ134–138 5 similar groups (5 cells; same shape, refs/constants differ) e.g. AJ3  =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!j3:j2000") [external] [array/spills]; others at AV3, BT3, DP3, EZ3
ƒ139–143 5 similar groups (5 cells; same shape, refs/constants differ) e.g. AK3  =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","1/2!e3:e2000") [external] [array/spills]; others at AW3, BU3, DQ3, FA3
ƒ144–146 3 similar groups (3 cells; same shape, refs/constants differ) e.g. AO3  =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","3/2!c3:f2000") [external] [array/spills]; others at DI3, ES3
ƒ147–149 3 similar groups (3 cells; same shape, refs/constants differ) e.g. AS3  =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","2/1!h3:h2000") [external] [array/spills]; others at DM3, EW3
ƒ150–152 3 similar groups (3 cells; same shape, refs/constants differ) e.g. AT3  =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","3/2!h3:h2000") [external] [array/spills]; others at DN3, EX3
ƒ153 BA3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","3/2!s3:v2000") [external] [array/spills]
ƒ154 BE3 =importrange("https://docs.google.com/spreadsheets/d/1enLc5rvVso8CAAgk11VBr_AD4NLNpxQNWatH801lC8o/edit#gid=1923577576","2/1!z3:z2000") [external] [array/spills]
ƒ155 BF3 =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","3/2!x3:x2000") [external] [array/spills]
ƒ156–159 4 similar groups (4 cells; same shape, refs/constants differ) e.g. DU3  =importrange("https://docs.google.com/spreadsheets/d/1heGSOxnuL0u3RyTLkhzMThh_ukC9z4uvC-2rjidCI_g/edit#gid=1218937572","7/2!k3:p2000") [external] [array/spills]; others at FE3, GC3, HM3
◐ conditional formats
O1:O3000, AA1:AA3000, AB1 +33: TEXT_EQ Correct → bg#B7E1CD
O1:O3000, AA1:AA3000, AB1 +33: TEXT_EQ Re-check → bg#EA9999
A1:A3000: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (440268 cells): D3:D2681, E4:E2681, F3:J2681, K4:M2681 +77
bg#EFEFEF (102024 cells): A3:C2681, N3:N2681, Y4:Y2681, Z3:Z2681 +36
bg#B7E1CD (48341 cells): O3:O2681, AA3:AA2681, AM3:AM2681, AY3:AY2681 +15
bg#FCE5CD bold (354 cells): D1:M2, P1:Y2, AB1:AK2, AN1:AW2 +24
bg#F9CB9C (84 cells): E3, K3:M3, Q3, W3:Y3 +34
bg#EFEFEF bold (77 cells): A1:C1, N1:O2, Z1:AA2, AL1:AM2 +18
bg#EFEFEF bold underline (3 cells): A2:C2
hidden rows: 4-309, 311-703, 705-732, 734-763, 766-1404
hidden cols: DI-DQ, HA-HI, HM-HW
✎ notes (2)
DC2: ของเก่าวันก่อนหน้า
DD2: ใช้จริงวันปัจจุบัน
