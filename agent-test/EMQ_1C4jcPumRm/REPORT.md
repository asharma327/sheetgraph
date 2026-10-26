# ใบตรวจการใช้วัตถุดิบ (EMQ)
gsheet 1C4jcPumRmTMZcmCdq5gVRdIu4wj60PhSegGDDCoudss · en_US · Asia/Bangkok · modified 2026-10-23 · owner accounting@hunterpoke.com
23 sheets · 2.6M non-empty cells · 1.3M formulas in 1824 copy-groups · 27k error cells
Notation: ƒN = formula copy-group N of a sheet; a group's formula is shown at its top-left cell and is copied (relative refs shift) over its range. ↓ filled down, → across, ▦ both.

## Sheets
1. Template — C1:C3 · 3 cells
2. Sales — A1:L2000 · 24k cells · 1 ƒ in 1 groups
3. Monthly Summary (Usage) — C1:BI249 · 8316 cells · 8118 ƒ in 241 groups · reads Sales, Item Sales (Cal), Item Sales (POS), Fish record, Salmon, Tuna, Chicken, Chicken BB, Avocado, Mango, MahachanokMango, Pomelo, Edamame, Tamago, Ikura, Others
4. Monthly Summary (item sales) — B1:CB280 · 11k cells · 10k ƒ in 198 groups · reads Item Sales (POS)
5. Item Sales (Cal) — A1:BPO716 · 894k cells · 893k ƒ in 406 groups · reads Item Sales (POS)
6. Item Sales (POS) — A1:BNX812 · 967k cells · 11k ƒ in 11 groups
7. Item Sales (Usage) — A1:M2000 · 24k cells · 22k ƒ in 13 groups · reads Item Sales (Cal)
8. Inventory Summary — A1:I34 · 117 cells · 78 ƒ in 78 groups · reads Fish record, Salmon, Tuna, Avocado, Mango, MahachanokMango, Pomelo, Edamame, Tamago, Eel, Ikura, Others
9. Monthly Inventory Summary — A1:AE35 · 941 cells · 835 ƒ in 429 groups · reads Salmon, Tuna, Avocado, Mango, MahachanokMango, Pomelo, Edamame, Tamago, Eel, Ikura, Others
10. Fish record — A1:W2000 · 30k cells · 12k ƒ in 9 groups
11. Salmon — A1:V2001 · 37k cells · 28k ƒ in 21 groups · reads Sales, Item Sales (Usage), Fish record
12. Tuna — A1:V2000 · 37k cells · 28k ƒ in 118 groups · reads Sales, Item Sales (Usage), Fish record
13. Chicken — A1:V2043 · 29k cells · 20k ƒ in 17 groups · reads Sales, Item Sales (Usage)
14. Chicken BB — A1:V2043 · 26k cells · 20k ƒ in 17 groups · reads Sales, Item Sales (Usage)
15. Avocado — A1:N2000 · 21k cells · 14k ƒ in 9 groups · reads Sales, Item Sales (Usage)
16. Mango — A1:N2000 · 21k cells · 14k ƒ in 10 groups · reads Sales, Item Sales (Usage)
17. MahachanokMango — A1:N2000 · 19k cells · 14k ƒ in 12 groups · reads Sales, Item Sales (Usage)
18. Pomelo — A1:V2000 · 32k cells · 20k ƒ in 16 groups · reads Sales, Item Sales (Usage)
19. Edamame — A1:V2000 · 31k cells · 20k ƒ in 16 groups · reads Sales, Item Sales (Usage)
20. Tamago — A1:V2005 · 32k cells · 20k ƒ in 16 groups · reads Sales, Item Sales (Usage)
21. Eel — A1:R2000 · 26k cells · 14k ƒ in 13 groups · reads Sales
22. Ikura — A1:V2000 · 30k cells · 20k ƒ in 16 groups · reads Sales, Item Sales (Usage)
23. Others — A1:HW2011 · 271k cells · 118k ƒ in 157 groups · reads Sales

## Data flow (source → readers, by formula refs)
Sales → Monthly Summary (Usage) (6g), Salmon (2g), Tuna (2g), Chicken (2g), Chicken BB (2g), Avocado (2g), Mango (2g), MahachanokMango (2g), Pomelo (2g), Edamame (2g), Tamago (1g), Eel (2g), Ikura (2g), Others (38g)
Item Sales (Cal) → Monthly Summary (Usage) (7g), Item Sales (Usage) (10g)
Item Sales (POS) → Monthly Summary (Usage) (2g), Monthly Summary (item sales) (75g), Item Sales (Cal) (365g)
Item Sales (Usage) → Salmon (1g), Tuna (1g), Chicken (1g), Chicken BB (1g), Avocado (1g), Mango (2g), MahachanokMango (1g), Pomelo (1g), Edamame (1g), Tamago (1g), Ikura (1g)
Fish record → Monthly Summary (Usage) (16g), Inventory Summary (6g), Salmon (5g), Tuna (104g)
Salmon → Monthly Summary (Usage) (4g), Inventory Summary (3g), Monthly Inventory Summary (14g)
Tuna → Monthly Summary (Usage) (4g), Inventory Summary (2g), Monthly Inventory Summary (14g)
Chicken → Monthly Summary (Usage) (14g)
Chicken BB → Monthly Summary (Usage) (1g)
Avocado → Monthly Summary (Usage) (8g), Inventory Summary (3g), Monthly Inventory Summary (7g)
Mango → Monthly Summary (Usage) (9g), Inventory Summary (3g), Monthly Inventory Summary (7g)
MahachanokMango → Monthly Summary (Usage) (8g), Inventory Summary (1g), Monthly Inventory Summary (7g)
Pomelo → Monthly Summary (Usage) (14g), Inventory Summary (2g), Monthly Inventory Summary (14g)
Edamame → Monthly Summary (Usage) (14g), Inventory Summary (5g), Monthly Inventory Summary (14g)
Tamago → Monthly Summary (Usage) (14g), Inventory Summary (5g), Monthly Inventory Summary (14g)
Eel → Inventory Summary (2g), Monthly Inventory Summary (14g)
Ikura → Monthly Summary (Usage) (14g), Inventory Summary (2g), Monthly Inventory Summary (14g)
Others → Monthly Summary (Usage) (36g), Inventory Summary (42g), Monthly Inventory Summary (259g)

## External links
IMPORTRANGE 19pXjj0ZWWhr3kcWDRrT3w3Dxd88elcT1x1rKM1I1W6M (1): "Sales(ใบตรวจร้าน)!A1:L2000"→Sales!A1
IMPORTRANGE 1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c (39): "Fish record summary!D3:g2000"→'Fish record'!D3, "Fish record summary!j3:m2000"→'Fish record'!J3, "1/1!c3:h2000"→Salmon!E4, "1/1!k3:k2000"→Salmon!V4, "1/2!c3:h11"→Tuna!E3, "1/2!c3:c2000"→Tuna!V3, "1/1!s12:x2000"→Tuna!E12, "9/2!K460:P2000"→Chicken!E3, "9/2!C460:H2000"→'Chicken BB'!E3, "2/1!c3:h2000"→Avocado!E3, "1/2!k3:p2000"→Mango!E3, "1/2!s3:x2000"→MahachanokMango!E3 +27
IMPORTRANGE 1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0 (87): "5/3!E460:E2000"→Chicken!K3, "5/3!J460:J2000"→Chicken!L3, "5/3!G460:G2000"→Chicken!M3, "5/3!H460:H2000"→Chicken!N3, "5/3!C460:C2000"→Chicken!O3, "5/3!N460:N2000"→'Chicken BB'!K3, "5/3!S460:S2000"→'Chicken BB'!L3, "5/3!P460:P2000"→'Chicken BB'!M3, "5/3!Q460:Q2000"→'Chicken BB'!N3, "5/3!L460:L2000"→'Chicken BB'!O3, "3/1!n3:n2000"→Pomelo!K3, "3/1!s3:s2000"→Pomelo!L3 +75

## Functions (cells using)
AND 130k · IF 82k · IFERROR 56k · OR 52k · YEAR 34k · MONTH 34k · SUMIFS 26k · HLOOKUP 18k · SUM 13k · DAY 1834 · SUMPRODUCT 1250 · INDEX 454 · MATCH 454 · IMPORTRANGE 127 · EOMONTH 33 · TODAY 2
volatile: TODAY 2

## Extraction notes
- Apps Script (bound scripts, macros, add-ons, triggers) is not exposed by the Sheets API; pass --apps-script <scriptId> with user credentials to include a project

## 1. Template · C1:C3 · 3 cells
▸ C1:C3 (3 cells)
  1: C=importrange
  2: C=Manual cal
  3: C=Formula

## 2. Sales · A1:L2000 · 24k cells
read by: Monthly Summary (Usage), Salmon, Tuna, Chicken, Chicken BB, Avocado, Mango, MahachanokMango, Pomelo, Edamame, Tamago, Eel, Ikura, Others
▸ A1:L2000 table 2000×12 (23989 cells), header row 3
  A "Date": date Tue 01-03-2022  … Wed 18-08-2027  (1997 filled)
  B "Month": num 1 … 12 (1997 filled)
  C "Year": num 2022 … 2027 (1997 filled)
  D "[17  / TCM] Total": num 0.00 … 50,292.00; 230 errors (1997 filled)
  E "[6 ] instore": num 0.00 … 23,785.00; 230 errors (1997 filled)
  F "[11 ] delivery": num 0.00 … 37,975.00; 230 errors (1997 filled)
  G "[33  / EMQ] Total": num 0.00 … 34,600.00; 230 errors (1997 filled)
  H "[20 ] instore": num 0.00 … 28,345.00; 230 errors (1997 filled)
  I "[26 ] delivery": num 0.00 … 23,435.00; 230 errors (1997 filled)
  J "[47  / CSD] Total": num 0.00 … 41,365.50; 230 errors (1997 filled)
  K "[36 ] instore": num 0.00 … 18,820.00; 230 errors (1997 filled)
  L "[41 ] delivery": num 0.00 … 35,300.00; 230 errors (1997 filled)
  sample (A…L):
  4| Tue 01-03-2022  | 3 | 2022 | 30,682.00 | 6,780.00 | 23,902.00 | 14,872.00 | 9,690.00 | 5,182.00 | 24,548.00 | 5,800.00 | 18,748.00
  5| Wed 02-03-2022  | 3 | 2022 | 34,837.00 | 8,980.00 | 25,857.00 | 15,002.00 | 8,550.00 | 6,452.00 | 30,578.00 | 6,786.00 | 23,792.00
  6| Thu 03-03-2022  | 3 | 2022 | 28,346.00 | 10,011.00 | 18,335.00 | 14,475.00 | 10,330.00 | 4,145.00 | 22,071.00 | 5,776.00 | 16,295.00
  2000| Wed 18-08-2027  | 8 | 2027 | #N/A | #N/A | #N/A | #N/A | #N/A | #N/A | #N/A | #N/A | #N/A
ƒ formulas (1 cells, 1 groups, 1 families)
ƒ1 A1 =importrange("https://docs.google.com/spreadsheets/d/19pXjj0ZWWhr3kcWDRrT3w3Dxd88elcT1x1rKM1I1W6M/edit#gid=35816577","Sales(ใบตรวจร้าน)!A1:L2000") [external] [array/spills]
◆ formatting
bg#FCE5CD (1 cells): A1

## 3. Monthly Summary (Usage) · C1:BI249 · 8316 cells · frozen 3r4c
reads: Sales (6g), Item Sales (Cal) (7g), Item Sales (POS) (2g), Fish record (16g), Salmon (4g), Tuna (4g), Chicken (14g), Chicken BB (1g), Avocado (8g), Mango (9g), MahachanokMango (8g), Pomelo (14g), Edamame (14g), Tamago (14g), Ikura (14g), Others (36g)
▸ C1:BI19 row-oriented 19×59 (1074 cells), labels in column C
  1 "Year": ƒ1; num 2022 … 2026 (57 filled)
  2 "Month": ƒ2; num 1 … 12 (57 filled)
  3 "Date": ƒ3; date 1/31/2022 … 9/30/2026 (57 filled)
  4 "Average daily sale": ƒ4; num 12,430.53 … 24,002.29 fmt #,##0.00 (55 filled)
  5 "EMQ Total Sales": ƒ5,ƒ6; num 372,916.00 … 744,071.00 fmt #,##0.00 (55 filled)
  6 "instore": ƒ7,ƒ8; num 170,140.00 … 435,807.00 fmt #,##0.00 (55 filled)
  7 "Delivery": ƒ9,ƒ10; num 170,760.00 … 450,346.00 fmt #,##0.00 (55 filled)
  8 "Salmon (g/1000baht)": ƒ136; num 85.46 ก. … 150.22 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (57 filled)
  9 "Total Salmon": ƒ11,ƒ12; num 39491 … 94598 fmt #,##0.00 (57 filled)
  10 "Yield": ƒ147; num 85.51% … 93.39% fmt 0.00% (55 filled)
  11 "waste": ƒ13,ƒ14; num 3075 … 10976 (55 filled)
  12 "closed": ƒ15,ƒ16; num 854 … 7381 (55 filled)
  13 "expired": ƒ154,ƒ155; num 0 (55 filled)
  14 "Total Salmon throw away (g)": ƒ156; num 3,929.00 … 18,357.00 fmt #,##0.00 (55 filled)
  15 "Total Salmon throw away(%)": ƒ163; num 8.54% … 24.24% fmt 0.00% (55 filled)
  16 "Raw mat - Prepared  g.ใบนับขอ…": ƒ17,ƒ18; num 2,121.00 … 5,263.00 fmt #,##0.00 (55 filled)
  17 "Raw mat - Prepared %ใบนับของส…": ƒ164; num 3.63% … 8.31% fmt 0.00% (55 filled)
  18 "Prepared usage - Theoratical …": ƒ19,ƒ20; num -106.00 … 2,217.00 fmt #,##0.00 (55 filled)
  19 "Prepared usage - Theoratical …": ƒ180; num -0.23% … 3.69% fmt 0.00% (55 filled)
▸ G21:BI27 table 7×55 (385 cells), no header row
  G…BI (55 alike): ƒ21,ƒ187,ƒ191 +4; num 122.67 ก. … 61,890.00 fmt #,##0.00 (7 filled)
  sample (G…V of 55):
  21| 61,890.00 | 60,435.00 | 55,890.00 | 49,995.00 | 46,995.00 | 60,045.00 | 53,640.00 | 62,685.00 | 70,905.00 | 73,665.00 | 77,280.00 | 55,410.00 | 53,445.00 | 46,845.00 | 58,215.00 | 54,060.00
  22| 151.05 ก. | 147.04 ก. | 121.36 ก. | 118.44 ก. | 132.16 ก. | 145.40 ก. | 145.48 ก. | 153.78 ก. | 152.78 ก. | 153.74 ก. | 155.81 ก. | 140.82 ก. | 127.15 ก. | 108.85 ก. | 114.53 ก. | 114.54 ก.
  23| 43,861.30 | 39,873.94 | 37,649.27 | 31,103.89 | 33,214.30 | 40,462.02 | 35,502.38 | 44,354.94 | 50,921.10 | 51,638.74 | 56,742.28 | 39,227.88 | 39,481.48 | 31,593.21 | 39,017.94 | 37,380.98
  27| 3239.57 | 3410.00 | 3305.46 | 3782.38 | 3902.40 | 3108.10 | 3662.01 | 3375.64 | 3402.66 | 4182.19 | 4039.24 | 5361.04 | 3105.18 | 1971.71 | 2184.67 | 2178.52
▸ C29:BI40 row-oriented 12×59 (676 cells), labels in column C
  29 "ทูน่า (กรัม/1000บาท)": ƒ137; num 34.77 ก. … 52.84 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (57 filled)
  30 "ทูน่าทั้งหมด": ƒ25,ƒ26; num 15075 … 39009 (57 filled)
  31 "Yield": ƒ148; num 98.65% … 100.00% fmt 0.00% (55 filled)
  32 "waste": ƒ27,ƒ28; num 0 … 322 (55 filled)
  33 "closed": ƒ29,ƒ30; num 173 … 3096 (55 filled)
  34 "expired": ƒ31,ƒ32; num 0 (55 filled)
  35 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ157; num 173.00 … 3,110.00 fmt #,##0.00 (55 filled)
  36 "ปริมาณทิ้งทั้งหมด (%)": ƒ165; num 0.64% … 12.41% fmt 0.00% (55 filled)
  37 "ใบนับของสด-ใบจดปลา (กรัม)": ƒ33,ƒ34; num 1,606.00 … 3,386.00 fmt #,##0.00 (55 filled)
  38 "ใบนับของสด-ใบจดปลา (%)": ƒ166; num 6.96% … 14.08% fmt 0.00% (55 filled)
  39 "ใบจดปลา-Item Sales(กรัม)": ƒ35,ƒ36; num -291.00 … 1,284.00 fmt #,##0.00 (55 filled)
  40 "ใบจดปลา-Item Sales (%)": ƒ181; num -1.29% … 6.07% fmt 0.00% (55 filled)
▸ G42:BI48 table 7×55 (385 cells), no header row
  G…BI (55 alike): ƒ37,ƒ189,ƒ192 +4; num 22.28 ก. … 19,275.00 fmt #,##0.00 (7 filled)
  sample (G…V of 55):
  42| 19,275.00 | 19,605.00 | 25,155.00 | 25,725.00 | 21,090.00 | 19,845.00 | 17,520.00 | 16,800.00 | 19,695.00 | 21,795.00 | 19,605.00 | 22,455.00 | 21,825.00 | 23,355.00 | 26,925.00 | 26,730.00
  43| 51.92 ก. | 53.37 ก. | 55.93 ก. | 65.27 ก. | 53.64 ก. | 50.95 ก. | 52.26 ก. | 44.98 ก. | 43.55 ก. | 46.69 ก. | 48.88 ก. | 65.22 ก. | 60.89 ก. | 73.70 ก. | 73.16 ก. | 73.80 ก.
  44| 15,835.71 | 14,875.92 | 18,476.54 | 18,352.78 | 15,548.00 | 14,499.27 | 13,393.49 | 13,548.57 | 15,208.10 | 16,958.13 | 19,605.00 | 22,455.00 | 21,825.00 | 23,355.00 | 26,925.00 | 26,730.00
  48| 423.06 | 634.40 | 730.10 | 891.26 | 624.36 | 711.69 | 596.37 | 435.27 | 524.44 | 486.02 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00
▸ C50:BI56 row-oriented 7×59 (396 cells), labels in column C
  50 "มะม่วง (กรัม/1000บาท)": ƒ138; num 0.00 ก. … 142.72 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (57 filled)
  51 "มะม่วงทั้งหมด": ƒ40,ƒ41,ƒ42; num 0.00 … 73,633.00 fmt #,##0.00 (57 filled)
  52 "Yield": ƒ197; num 26.04% … 195.26% fmt 0.00%; 2 errors (55 filled)
  53 "waste ใบของสด": ƒ43,ƒ44; num 0 … 16713 (55 filled)
  54 "expired": ƒ45,ƒ46; num 0 … 1904 (55 filled)
  55 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ47,ƒ48; num -19,932.00 … 47,422.00 fmt #,##0.00 (55 filled)
  56 "ปริมาณทิ้งทั้งหมด (%)": ƒ167; num -95.26% … 71.56% fmt 0.00%; 2 errors (55 filled)
▸ C59:BI124 row-oriented 66×59 (3704 cells), labels in column C
  59 "อโวคาโด (กรัม/1000บาท)": ƒ139; num 65.60 ก. … 111.39 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (57 filled)
  60 "อโวคาโดทั้งหมด": ƒ49,ƒ50; num 33,260.00 … 67,020.00 fmt #,##0.00 (57 filled)
  61 "Yield": ƒ198; num 33.99% … 53.14% fmt 0.00% (55 filled)
  62 "waste ใบของสด": ƒ51,ƒ52; num 0 … 17021 (55 filled)
  63 "expired": ƒ53,ƒ54; num 0 … 267 (55 filled)
  64 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ55,ƒ56; num 17,803.00 … 42,932.00 fmt #,##0.00 (55 filled)
  65 "ปริมาณทิ้งทั้งหมด (%)": ƒ168; num 46.86% … 66.01% fmt 0.00% (55 filled)
  66 "มะม่วงมหาชนก (กรัม/1000บาท)": ƒ140,ƒ141; num 0.00 ก. … 107.32 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (55 filled)
  67 "มะม่วงมหาชนกทั้งหมด": ƒ57,ƒ58; num 0.00 … 63,660.00 fmt #,##0.00 (55 filled)
  68 "Yield": ƒ199; num 31.32% … 331.38% fmt 0.00%; 42 errors (55 filled)
  69 "waste ใบของสด": ƒ59,ƒ60; num 0 … 2480 (55 filled)
  70 "expired": ƒ61,ƒ62; num 0.00% … 40800.00% fmt 0.00% (55 filled)
  71 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ63,ƒ64; num -25,322.00 … 41,261.00 fmt #,##0.00 (55 filled)
  72 "ปริมาณทิ้งทั้งหมด (%)": ƒ169; num -231.38% … 68.40% fmt 0.00%; 42 errors (55 filled)
  73 "ส้มโอ (กรัม/1000บาท)": ƒ142; num 14.89 ก. … 35.21 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (57 filled)
  74 "ส้มโอ ทั้งหมด": ƒ65,ƒ66; num 6,794.00 … 18,319.00 fmt #,##0.00 (57 filled)
  75 "Yield": ƒ149; num 91.73% … 98.19% fmt 0.00% (55 filled)
  76 "waste ใบของสด": ƒ67,ƒ68; num 191 … 1451 (55 filled)
  77 "expired ใบของสด": ƒ69,ƒ70; num 211 … 2503 (55 filled)
  78 "waste ใบเตรียมของ": ƒ71,ƒ72; num 0 (55 filled)
  79 "expired ใบเตรียมของ": ƒ73,ƒ74; num 98 … 3797 (55 filled)
  80 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ158; num 1,164.00 … 4,927.00 fmt #,##0.00 (55 filled)
  81 "ปริมาณทิ้งทั้งหมด (%)": ƒ170; num 8.27% … 34.35% fmt 0.00% (55 filled)
  82 "ใบนับของสด-ใบเตรียมของ (กรัม)": ƒ75,ƒ76; num 191.00 … 1,451.00 fmt #,##0.00 (55 filled)
  83 "ใบนับของสด-ใบเตรียมของ (%)": ƒ171; num 1.81% … 8.27% fmt 0.00% (55 filled)
  84 "ใช้ไปใบเตรียมของ-Item Sales(ก…": ƒ77,ƒ78; num -17.00 … 2,370.00 fmt #,##0.00 (55 filled)
  85 "ใช้ไปใบเตรียมของ-Item Sales (…": ƒ182; num -0.20% … 14.35% fmt 0.00% (55 filled)
  86 "ถั่วแระ (กรัม/1000บาท)": ƒ143; num 46.50 ก. … 91.10 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (55 filled)
  87 "ถั่วแระ ทั้งหมด": ƒ79,ƒ80; num 18,000.00 … 48,000.00 fmt #,##0.00 (55 filled)
  88 "Yield": ƒ150; num 39.54% … 50.51% fmt 0.00% (55 filled)
  89 "waste ใบของสด": ƒ81,ƒ82; num 9771 … 26740 (55 filled)
  90 "expired ใบของสด": ƒ83,ƒ84; num 0 (55 filled)
  91 "waste ใบเตรียมของ": ƒ85,ƒ86; num 0 (55 filled)
  92 "expired ใบเตรียมของ": ƒ87,ƒ88; num 0 … 1117 (55 filled)
  93 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ159; num 9,835.00 … 26,740.00 fmt #,##0.00 (55 filled)
  94 "ปริมาณทิ้งทั้งหมด (%)": ƒ172; num 50.78% … 60.46% fmt 0.00% (55 filled)
  95 "ใบนับของสด-ใบเตรียมของ (กรัม)": ƒ89,ƒ90; num 9,771.00 … 26,740.00 fmt #,##0.00 (55 filled)
  96 "ใบนับของสด-ใบเตรียมของ (%)": ƒ173; num 49.49% … 60.46% fmt 0.00% (55 filled)
  97 "ใบเตรียมของ-Item Sales(กรัม)": ƒ91,ƒ92; num -239.00 … 1,874.00 fmt #,##0.00 (55 filled)
  98 "ใบเตรียมของ-Item Sales (%)": ƒ183; num -1.56% … 9.57% fmt 0.00% (55 filled)
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
  V…AY (30 alike): ƒ146,ƒ121,ƒ153 +10; num 0 … 8,000.00 fmt #,##0.00 (13 filled)
  AZ…BC (4 alike): ƒ146,ƒ122,ƒ153 +10; num 0.00 ก. fmt #,##0.00; 4 errors (13 filled)
  BD…BI (6 alike): ƒ146,ƒ122,ƒ153 +10; num -4,590.00 … 2,000.00 fmt #,##0.00 (13 filled)
  sample (V…AK of 40):
  126| 13.56 ก. | 12.13 ก. | 14.44 ก. | 15.94 ก. | 14.48 ก. | 15.19 ก. | 19.86 ก. | 17.00 ก. | 13.76 ก. | 19.68 ก. | 19.35 ก. | 25.98 ก. | 23.11 ก. | 32.49 ก. | 21.00 ก. | 26.81 ก.
  127| 8,000.00 | 7,000.00 | 8,000.00 | 8,000.00 | 7,000.00 | 8,000.00 | 9,000.00 | 9,000.00 | 7,000.00 | 11,000.00 | 10,000.00 | 14,000.00 | 13,000.00 | 18,000.00 | 12,000.00 | 13,000.00
  128| 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 101.49% | 110.50% | 109.38% | 104.39% | 101.61% | 111.00% | 107.05%
  138| 0.00% | -0.86% | 0.00% | 0.00% | 0.00% | 0.00% | -0.67% | 0.00% | -1.43% | -0.35% | 0.62% | 1.72% | 1.26% | 1.22% | -0.47% | 0.74%
▸ AO140:BI141 (23 cells)
  140: AO=ƒ135→0.00 AP=ƒ135→0.00 AQ=ƒ135→0.00 AR=ƒ135→0.00 AS=ƒ135→0.00 AT=ƒ135→0.00 AU=ƒ135→0.00 AV=ƒ135→0.00 AW=ƒ135→5,600.00 AX=ƒ135→12,000.00 AY=ƒ135→14,865.00 AZ=ƒ135→26,000.00 BA=ƒ135→34,000.00 BB=ƒ135→33,000.00 BC=ƒ135→35,000.00 BD=ƒ135→35,000.00 BE=ƒ135→16,000.00 BF=ƒ135→31,700.00 BG=ƒ135→31,582.00 BH=ƒ135→38,000.00 BI=ƒ135→36,000.00
  141: BG=ƒ200→57,232.00 BH=ƒ200→73,240.00
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
  G…BI (55 alike): ƒ201,ƒ203,ƒ205 +15; num 0.33 ก. … 19.46 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (18 filled)
  sample (G…V of 55):
  143| 2.13 ก. | 2.52 ก. | 2.09 ก. | 1.92 ก. | 1.94 ก. | 1.79 ก. | 2.06 ก. | 1.71 ก. | 2.76 ก. | 1.55 ก. | 1.81 ก. | 1.79 ก. | 1.83 ก. | 1.61 ก. | 2.54 ก. | 2.08 ก.
  144| 1.36 ก. | 1.74 ก. | 1.18 ก. | 1.99 ก. | 1.38 ก. | 1.88 ก. | 1.68 ก. | 1.56 ก. | 1.03 ก. | 1.45 ก. | 1.11 ก. | 1.25 ก. | 1.26 ก. | 0.95 ก. | 1.10 ก. | 1.02 ก.
  145| 0.33 ก. | 0.37 ก. | 0.22 ก. | 0.40 ก. | 0.34 ก. | 0.32 ก. | 0.42 ก. | 0.31 ก. | 0.31 ก. | 0.43 ก. | 0.41 ก. | 0.30 ก. | 0.30 ก. | 0.30 ก. | 0.25 ก. | 0.28 ก.
  160| 2.67 ก. | 0.87 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.03 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก.
▸ G167:BI168 table 2×55 (110 cells), no header row
  G…BI (55 alike): ƒ237,ƒ238; num 6.03% … 10.50% fmt 0.00% (2 filled)
  sample (G…V of 55):
  167| 6.03% | 6.31% | 8.35% | 9.52% | 11.26% | 6.84% | 9.69% | 8.35% | 6.06% | 6.22% | 6.50% | 11.40% | 6.80% | 2.95% | 1.81% | 3.83%
  168| 10.50% | 10.82% | 9.63% | 10.59% | 9.84% | 11.24% | 12.22% | 11.27% | 10.09% | 8.85% | 7.74% | 7.00% | 5.31% | 3.05% | 2.92% | 1.59%
▸ +5 smaller blocks (21 cells): C21:C27, C42:C48, C166:C168, C247:C249, C140
ƒ formulas (8118 cells, 241 groups, 20 families)
ƒ1 E1:BI1 → =year(E3)
ƒ2 E2:BI2 → =month(E3)
ƒ3 AC3:BI3 → =EOMONTH(AB3,1)
ƒ4 G4:BI4 → =G5/day(G3)
ƒ5–135 131 similar groups (3491 cells; same shape, refs/constants differ) e.g. G5:AM5 → =sumifs(Sales!$G$3:$G$1000,Sales!$B$3:$B$1000,G$2,Sales!$C$3:$C$1000,G$1); others at AN5:BI5, G6:AM6, AN6:BI6, G7:AM7, AN7:BI7, G9:AM9 +124
ƒ136–146 11 similar groups (535 cells; same shape, refs/constants differ) e.g. G8:BI8 → =G9/G5*1000; others at G29:BI29, G50:BI50, G59:BI59, G66:H66, I66:BI66, G73:BI73 +4
ƒ147–153 G10:BI10 → =(G9-G11)/G9 — same formula in 7 ranges (370 cells): G31:BI31, G75:BI75, G88:BI88, G101:BI101, G114:BI114, V128:BI128; breaks at G11:BI30, G32:BI74, G76:BI87, G89:BI100, G102:BI113 = 5115 different formulas, 330 blank
ƒ154 G13:AM13 → =sumifs('Fish record'!$G$3:$G$1000,'Fish record'!$B$3:$B$1000,G$2,'Fish record'!$C$3:$C$1000,G$1)*-1
ƒ155 AN13:BI13 → =sumifs('Fish record'!$G$3:$G$2000,'Fish record'!$B$3:$B$2000,AN$2,'Fish record'!$C$3:$C$2000,AN$1)*-1
ƒ156–162 7 similar groups (370 cells; same shape, refs/constants differ) e.g. G14:BI14 → =sum(G11:G13); others at G35:BI35, G80:BI80, G93:BI93, G106:BI106, G119:BI119, V133:BI133
ƒ163–179 17 similar groups (905 cells; same shape, refs/constants differ) e.g. G15:BI15 → =G14/G9; others at G17:BI17, G36:BI36, G38:BI38, G56:BI56, G65:BI65, G72:BI72 +10
ƒ180–186 7 similar groups (370 cells; same shape, refs/constants differ) e.g. G19:BI19 → =G18/(G9-G11-G12); others at G40:BI40, G85:BI85, G98:BI98, G111:BI111, G124:BI124, V138:BI138
ƒ187–190 4 similar groups (220 cells; same shape, refs/constants differ) e.g. G22:BI22 → =(G23+G24)/G$6*1000; others at G25:BI25, G43:BI43, G46:BI46
ƒ191–192 G23:BI23 → =G21-G26 — same formula in 2 ranges (110 cells): G44:BI44; breaks at G24:BI43 = 990 different formulas, 110 blank
ƒ193–196 4 similar groups (220 cells; same shape, refs/constants differ) e.g. G24:BI24 → =(G23/G21)*G14; others at G27:BI27, G45:BI45, G48:BI48
ƒ197–199 G52:BI52 → =(G51-G55-G54)/G51 [2 errors] — same formula in 3 ranges (165 cells): G61:BI61, G68:BI68; breaks at G53:BI60, G62:BI67 = 660 different formulas, 110 blank
ƒ200 BG141:BH141 → =sum(BG127,BG140)
ƒ201–236 36 similar groups (990 cells; same shape, refs/constants differ) e.g. G143:AM143 → =SUMIFS(Others!$J$3:$J$1000,Others!$B$3:$B$1000,G$2,Others!$C$3:$C$1000,G$1)/G$5*1000; others at AN143:BI143, G144:AM144, AN144:BI144, G145:AM145, AN145:BI145, G146:AM146 +29
ƒ237 G167:BI167 → =G12/(G9-G11)
ƒ238 G168:BI168 → =G33/(G30-G32)
ƒ240 C248 ='Item Sales (POS)'!C367
ƒ241 C249 ='Item Sales (POS)'!C376
typed arithmetic: 1 cells hold ref-free formulas (hand-entered sums) e.g. `=#REF!` in columns C
◆ formatting
fg#434343 italic (3847 cells): C10:BI10, C14:C19, C23:BI24, C26:C27 +52
italic (2104 cells): C11:BI13, C32:BI34, C53:D54, C62:C63 +14
bg#FFF2CC (612 cells): C50:BI51, C66:BI67, C73:BI74, C99:BI100 +3
bg#C9DAF8 (236 cells): C5:BI7, C142:BI142
bg#D9EAD3 (236 cells): C59:BI60, C86:BI87
bg#EFEFEF bold (177 cells): C1:BI3
bg#F9CB9C (120 cells): C112:BI113, D22, D25
bg#FCE5CD (118 cells): C8:BI9
+8 more styles

## 4. Monthly Summary (item sales) · B1:CB280 · 11k cells · frozen 3r3c
reads: Item Sales (POS) (75g)
▸ C1:BB3 row-oriented 3×52 (153 cells), labels in column C
  1 "Year": ƒ1; num 2022 … 2026 (50 filled)
  2 "Month": ƒ2,ƒ3; num 1 … 12 (50 filled)
  3 "Date": date 3/31/2022 … 4/30/2026 (50 filled)
▸ C5:BB32 row-oriented 28×52 (1183 cells), labels in column C
  5 -: ƒ4; num -30.42% … 21.86% fmt 0.00% (32 filled)
  6 "In-store": ƒ20,ƒ21; num 451.5 … 1207.5 (50 filled)
  7 "The Classic": ƒ33; num 77 … 322.5 (50 filled)
  8 "BYO": ƒ34; num 87 … 211 (50 filled)
  9 "X Factor": ƒ35,ƒ36; num 60 … 267 (50 filled)
  10 "California": ƒ37; num 20 … 314.5 (50 filled)
  11 "Sexy Salmon": ƒ38; num 4 … 41.5 (50 filled)
  12 "K bowl": ƒ39; num 12 … 56.5 (50 filled)
  13 "Triple sea": ƒ40,ƒ41; num 0 … 44.5 (50 filled)
  14 "Pimpim pesto": ƒ42,ƒ43; num 0 … 14 (50 filled)
  15 "Chipotle": ƒ44,ƒ45; num 5 … 45 (50 filled)
  16 "Hunter's bowl": ƒ46,ƒ47; num 1 … 25 (50 filled)
  17 "Tofu emperor": ƒ48; num 5 … 28 (50 filled)
  18 "Avo-mango balsamic": ƒ49; num 7 … 35 (50 filled)
  19 "Triple sea (no engawa)": ƒ50; num 0 (50 filled)
  20 "Chicken bowl": ƒ22; num 4 … 26 (35 filled)
  21 "instore Chicken Rosa": ƒ112; num 0 … 13 (36 filled)
  22 "instore Chicken Black Truffle": ƒ112; num 0 … 14 (36 filled)
  23 "instore Chicken Tom Yum": ƒ112; num 0 … 14 (36 filled)
  24 "Wrap": ƒ23,ƒ24,ƒ25; num 13 … 114 (31 filled)
  25 "IN Chicken Chipotle Wrap ": ƒ118,ƒ119,ƒ120 +1; num 7 … 55 (35 filled)
  26 "IN CBT Wrap ": ƒ118,ƒ119,ƒ120 +1; num 0 … 7 (35 filled)
  27 "IN Rainbow Wrap": ƒ118,ƒ119,ƒ120 +1; num 1 … 12 (35 filled)
  28 "IN kimchi wrap": ƒ118,ƒ119,ƒ120 +1; num 0 … 17 (35 filled)
  29 "IN Guacamole wrap": ƒ118,ƒ119,ƒ120 +1; num 0 … 10 (35 filled)
  30 "IN Caesar wrap": ƒ119,ƒ120,ƒ113; num 2 … 17 (27 filled)
  31 "IN Chicken Rosa Wrap": ƒ120,ƒ113; num 0 … 4 (24 filled)
  32 "IN Chicken Seafood Pesto Wrap": ƒ120,ƒ113; num 0 … 3 (24 filled)
▸ BI5:BP20 table 16×8 (106 cells), no header row
  BI…BP (7 alike): ƒ8,ƒ64; num 0.00% … 750 fmt 0.00% (15 filled)
  sample (BI…BP):
  5|  |  |  |  |  |  | 32024 | 
  6| 750 | 732 |  | 747.5 | 739 | 676.5 | 747 | 763.5
  7| 15.67% | 16.80% |  | 16.32% | 19.55% | 19.73% | 14.79% | 15.85%
  20| 3.07% | 2.87% |  | 3.21% | 1.62% | 1.18% | 3.08% | 1.31%
▸ BF6:BF20 (15 cells)
  6: BF=ƒ7→747.5
  7: BF=ƒ63→19.26%
  8: BF=ƒ63→15.12%
  9: BF=ƒ63→16.39%
  10: BF=ƒ63→22.54%
  11: BF=ƒ63→2.14%
  12: BF=ƒ63→1.61%
  13: BF=ƒ63→1.74%
  14: BF=ƒ63→0.00%
  15: BF=ƒ63→3.08%
  16: BF=ƒ63→1.27%
  17: BF=ƒ63→2.01%
  18: BF=ƒ63→2.81%
  19: BF=ƒ63→0.00%
  20: BF=ƒ63→1.07%
▸ BT7:CB20 table 14×9 (122 cells), no header row
  BT…BW (4 alike): ƒ71; num 0.00% … 24.73% fmt 0.00% (14 filled)
  BX -: text 14 distinct e.g. "Avo-mango balsamic", "BYO", "California" (14 filled)
  BY…CB (4 alike): num 0.00% … 24.73% fmt 0.00% (13 filled)
  sample (BT…CB):
  7| 21.54% | 19.62% | 16.34% | 15.27% | The Classic | 21.54% | 19.62% | 16.34% | 15.27%
  8| 17.29% | 19.97% | 19.74% | 19.42% | Chicken bowl | 1.06% | 0.70% | 1.13% | 0.84%
  9| 19.41% | 17.30% | 17.41% | 18.65% | K bowl | 3.32% | 3.80% | 2.70% | 3.17%
  20| 1.06% | 0.70% | 1.13% | 0.84% | Wrap | 0.00% | 1.83% | 3.02% | 6.33%
▸ C34:BB60 row-oriented 27×52 (1117 cells), labels in column C
  34…51 (18 alike) "" "Delivery" "The Classic" "BYO" "X Factor" "California" "Sexy Salmon" "K bowl" "Triple sea" "Pimpim pesto" "Chipotle" "Hunter's bowl" "Tofu emperor" "Avo-mango balsamic" "Chicken bowl" "Deli Chicken Rosa" "Deli Chicken Black Truffle" "De…: ƒ5; num -20.46% … 36.30% fmt 0.00% (31 filled)
  52 "Wrap": ƒ29,ƒ30; num 14 … 166 (31 filled)
  53 "Chicken Chipotle Wrap ": ƒ115,ƒ116,ƒ117 +1; num 0 … 77 (32 filled)
  54 "CBT Wrap ": ƒ115,ƒ116,ƒ117 +1; num 0 … 25 (32 filled)
  55 "Rainbow Wrap": ƒ115,ƒ116,ƒ117 +1; num 0 … 29 (32 filled)
  56 "Kimchi wrap": ƒ115,ƒ116,ƒ117 +1; num 0 … 21 (32 filled)
  57 "Guacamole wrap": ƒ115,ƒ116,ƒ117 +1; num 0 … 13 (32 filled)
  58 "Caesar wrap": ƒ116,ƒ117,ƒ18; num 2 … 22 (27 filled)
  59 "Chicken Rosa Wrap": ƒ117,ƒ18; num 1 … 19 (24 filled)
  60 "Chicken Seafood Pesto Wrap": ƒ117,ƒ18; num 0 … 5 (24 filled)
▸ BI35:BP48 table 14×8 (98 cells), no header row
  BI…BP (7 alike): ƒ14,ƒ88; num 0.00% … 600 fmt 0.00% (14 filled)
  sample (BI…BP):
  35| 600 | 600 |  | 575.5 | 500 | 520 | 559 | 468.5
  36| 20.33% | 19.42% |  | 22.50% | 26.30% | 27.12% | 26.57% | 24.01%
  37| 16.17% | 17.83% |  | 21.89% | 27.00% | 19.23% | 22.18% | 19.64%
  48| 0.00% | 0.00% |  | 0.00% | 0.00% | 0.00% | 0.00% | 0.00%
▸ BT36:CB49 table 14×9 (118 cells), no header row
  BT…BW (4 alike): ƒ95; num 0.00% … 26.64% fmt 0.00% (13 filled)
  BX -: text 14 distinct e.g. "Avo-mango balsamic", "BYO", "California" (14 filled)
  BY…CB (4 alike): num 0.00% … 26.64% fmt 0.00% (13 filled)
  sample (BT…CB):
  36| 25.59% | 30.95% | 24.49% | 25.82% | The Classic | 25.59% | 30.95% | 24.49% | 25.82%
  37| 22.77% | 19.23% | 21.22% | 24.38% | Avo-mango balsamic | 0.94% | 0.47% | 0.68% | 0.52%
  38| 26.64% | 25.09% | 21.67% | 17.43% | Triple sea | 0.00% | 0.00% | 0.23% | 0.79%
  49|  |  |  |  | Wrap | 0.00% | 3.28% | 7.45% | 11.53%
▸ B62:BB103 table 42×53 (2122 cells), no header row
  B -: num 1 … 1.5 (39 filled)
  C -: text 40 distinct e.g. "Avo-Mango Balsamic Sala…", "Avo-Mango Balsamic Sala…", "Avo-Mango Balsamic Sala…" (40 filled)
  E…BB (50 alike): ƒ122,ƒ123,ƒ124 +6; num 0 … 200 (39 filled)
  sample (B…Q of 53):
  62|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  63|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  64|  | Instore |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  103| 1.5 | Mayongchid Summer Bowl … |  | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4
▸ B105:BB140 table 36×53 (1821 cells), no header row
  B -: num 1 … 1.5 (35 filled)
  C -: text 36 distinct e.g. "Avo-mango Deli - พิเศษ", "Avo-mango Deli - สแตนดา…", "BYO Deli" (36 filled)
  E…BB (50 alike): ƒ131,ƒ132,ƒ133 +9; num 0 … 94 (35 filled)
  sample (B…Q of 53):
  105|  | Delivery |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  106| 1 | Classic Deli - สแตนดาร์ด |  | 94 | 97 | 96 | 107 | 75 | 85 | 88 | 104 | 114 | 98 | 126 | 80 | 65
  107| 1 | Classic Deli - พิเศษ |  | 33 | 40 | 33 | 43 | 29 | 42 | 24 | 33 | 28 | 41 | 32 | 27 | 20
  140| 1 | Mayongchid Deli |  | 8 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4
▸ C142:BB280 table 139×52 (4072 cells), no header row
  C -: text 139 distinct e.g. "Acai Base (Small)", "Acai Base (frozen) Deli…", "Berry Banana (frozen) D…" (139 filled)
  E…BB (50 alike): ƒ143,ƒ144,ƒ145 +1; num 0 … 15 (7 filled)
  sample (C…R of 52):
  142| Promotion |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  143| Spicy Sriracha (Grab 28… |  | 2 | 5 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  144| Spicy Sriracha in store |  | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 5 | 9 | 9 | 6 | 13 | 8 | 4
  280| Venice Frozen - Full |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
▸ BR227:BU231 (20 cells)
  227: BR=ƒ198→1900 BS=ƒ198→3040 BT=ƒ198→4370 BU=ƒ198→3990
  228: BR=ƒ198→440 BS=ƒ198→2420 BT=ƒ198→1980 BU=ƒ198→880
  229: BR=ƒ198→600 BS=ƒ198→1500 BT=ƒ198→1800 BU=ƒ198→2100
  230: BR=ƒ198→0 BS=ƒ198→0 BT=ƒ198→760 BU=ƒ198→950
  231: BR=ƒ198→0 BS=ƒ198→0 BT=ƒ198→220 BU=ƒ198→440
▸ +11 smaller blocks (63 cells): BF35:BF48, BT34:CB34, BR214:BU215, BI24:BP24, BI52:BP52, BM227:BM231, BT5:BW5, BT24:BW24 +3
ƒ formulas (10k cells, 198 groups, 13 families)
ƒ1 E1:BB1 → =year(E3)
ƒ2–3 E2:AT2 → =month(E3) — same formula in 2 ranges (46 cells): AY2:BB2; breaks at AU2:AX2 = 4 hardcoded values
ƒ4–5 W5:BB5 → =(W6-V6)/V6 — same formula in 2 ranges (63 cells): X34:BB34
ƒ6–19 14 similar groups (203 cells; same shape, refs/constants differ) e.g. BT5:BW5 → =W6+W20+W24; others at BF6, BI6, BJ6, BL6, BM6:BP6, BT34:BW34 +7
ƒ20–32 13 similar groups (272 cells; same shape, refs/constants differ) e.g. E6:AH6 → =sum(E7:E18); others at AI6:BB6, T20:BB20, X24:AA24, AB24:AD24, AE24:BB24, BY34:CB34 +6
ƒ33–62 30 similar groups (1250 cells; same shape, refs/constants differ) e.g. E7:BB7 → =sumproduct($B65:$B67,E65:E67); others at E8:BB8, E9:AH9, AI9:BB9, E10:BB10, E11:BB11, E12:BB12 +23
ƒ63–111 49 similar groups (379 cells; same shape, refs/constants differ) e.g. BF7:BF20 ↓ =AH7/$BF$6; others at BI7:BI20, BJ7:BJ20, BL7:BL20, BM7:BM20, BN7:BN20, BO7:BO20 +42
ƒ112–117 6 similar groups (451 cells; same shape, refs/constants differ) e.g. S21:BB23 ▦ =S189+S192; others at AK25:BB32, S49:BB51, W53:AA57, AB53:AD58, AE53:AJ60
ƒ118–120 T25:AA29 ▦ =T211 — same formula in 3 ranges (106 cells): AB25:AD30, AE25:AJ32
ƒ121 X63:BB63 → =X67+X70+X76+X73+X79+X82+X85+X88+X91+X94+X97+X100+X103
ƒ122–196 75 similar groups (7493 cells; same shape, refs/constants differ) e.g. E65:BB67 ▦ =sumifs('Item Sales (POS)'!7:7,'Item Sales (POS)'!$2:$2,E$2,'Item Sales (POS)'!$1:$1,E$1); others at E68:BB70, E71:BB73, E74:BB76, E77:BB79, E80:BB82, E83:BB100 +68
ƒ197 T202:BB204 ▦ =T189+T192+T195+T198
ƒ198 BR227:BU231 ▦ =X227*$BM227
◆ formatting
bg#D9EAD3 (5056 cells): B106:BP140, B146:BP147, B152:D153, B159:BP161 +17
bg#FFF2CC (4806 cells): B65:BP103, B176:BP179, B189:BP191, B211:B213 +13
bg#F4CCCC (937 cells): B20:BB20, B24:BB24, B48:BB48, B52:BB52 +12
bg#FCE5CD (818 cells): B165:B171, B192:U194, B208:W210, C165:C167 +6
bg#D0E0E3 (780 cells): B162:BP164, B174:BP175, B182:BP183, B198:BP200 +4
bg#D9D2E9 (403 cells): B150:BP151, C168:C171, C258:C266, AC258:BB266 +1
bg#CFE2F3 (387 cells): C267:C280, AA275:AB280, AC267:AE271, AC273:AE280 +1
bg#B6D7A8 (256 cells): C243:C250, X243:BB250
+9 more styles

## 5. Item Sales (Cal) · A1:BPO716 · 894k cells · frozen 4r4c
reads: Item Sales (POS) (365g)
read by: Monthly Summary (Usage), Item Sales (Usage)
▸ A1:BPO716 table 716×1783 (893643 cells), no header row
  A -: ƒ4; num 1 … 714 (714 filled)
  B -: num 24  … 60  fmt #,##0_);[Red](#,##0); enum s(75) t(30) st(17) (128 filled)
  C -: text 431 distinct e.g. "Hunter’s Bowl - พิเศษ", "Hunter’s Bowl - สแตนดาร…", "The Classic Chicken Pok…" (575 filled)
  D -: num (45) … 200 fmt #,##0_);[Red](#,##0) (308 filled)
  E…AEK (813 alike): ƒ1,ƒ2,ƒ5 +105; date Sun,  ,  … 1-Mar-22; num (30) … 2022 (462 filled)
  AEL -: ƒ1,ƒ2,ƒ3 +118; date Sun,  ,  … 22-May-24; num (300) … 2024; 2 errors (487 filled)
  AEM…AQI (309 alike): ƒ1,ƒ2,ƒ3 +118; date Mon,  ,  … 23-May-24; num (60) … 2024 (487 filled)
  AQJ -: ƒ1,ƒ2,ƒ3 +133; date Sat,  ,  … 28-Mar-25; num 0 … 2025; 2 errors (516 filled)
  AQK…BIG (465 alike): ƒ1,ƒ2,ƒ3 +133; date Sun,  ,  … 29-Mar-25; num (180) … 2025 (516 filled)
  BIH -: ƒ1,ƒ2,ƒ3 +151; date Sat,  ,  … 7-Jul-26; num 0 … 2026; 2 errors (543 filled)
  BII -: ƒ1,ƒ2,ƒ3 +151; date Sun,  ,  … 8-Jul-26; num 0 … 2026; 2 errors (543 filled)
  BIJ -: ƒ1,ƒ2,ƒ3 +151; date Mon,  ,  … 9-Jul-26; num 0 … 2026; 2 errors (543 filled)
  BIK…BMK (105 alike): ƒ1,ƒ2,ƒ3 +151; date Tue,  ,  … 10-Jul-26; num (60) … 2026 (543 filled)
  BML…BPO (82 alike): ƒ1,ƒ2,ƒ3 +151; date Mon,  ,  … 23-Oct-26; num 0 … 2026; 2 errors (543 filled)
  sample (A…P of 1783):
  1|  |  |  |  | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022
  2|  |  |  |  | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3
  3| 1 |  | วันที่ |  | 1-Mar-22 | 2-Mar-22 | 3-Mar-22 | 4-Mar-22 | 5-Mar-22 | 6-Mar-22 | 7-Mar-22 | 8-Mar-22 | 9-Mar-22 | 10-Mar-22 | 11-Mar-22 | 12-Mar-22
  716| 714 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
ƒ formulas (893k cells, 406 groups, 20 families)
ƒ1 E1:BPO1 → =year(E3)
ƒ2 E2:BPO2 → =month(E3)
ƒ3 F3:BPO3 → =E3+1
ƒ4 A6:A716 ↓ =A5+1
ƒ5–6 E4:QS4 → =day(E3) — same formula in 2 ranges (1778 cells): QU4:BPO4; breaks at QT4 = 1 different formulas
ƒ7 QT4 =DAY(QT3)
ƒ8–159 152 similar groups (689073 cells; same shape, refs/constants differ) e.g. E6:BNF13 ▦ ='Item Sales (POS)'!E6*$D6; others at BNG6:BPO13, E14:BNF17, BNG14:BPO17, E18:BNF48, BNG18:BPO48, E49:BNF50 +145
ƒ160–291 132 similar groups (132697 cells; same shape, refs/constants differ) e.g. VS64:BNF64 → =('Item Sales (POS)'!VS71+'Item Sales (POS)'!VS79)*$D64; others at BNG64:BPO64, VU172:WX172, E189:BNF189, BNG189:BPO189, VU190:WX190, VS198:BNF198 +125
ƒ292–313 22 similar groups (6363 cells; same shape, refs/constants differ) e.g. AKS65:BNF65 → =('Item Sales (POS)'!AKS89)*$D65; others at BNG65:BPO65, AKS199:BNF199, BNG199:BPO199, ARZ200:BNF201, BNG200:BPO201, AXD463:BCA464 +15
ƒ314 E439:BPO441 ▦ =sumifs(E$6:E$435,$B$6:$B$435,$B439)
ƒ315 E442:BPO442 → =E439+E441
ƒ316 E443:BPO443 → =E440+E441
ƒ317–344 28 similar groups (12347 cells; same shape, refs/constants differ) e.g. E445:AE445 → =sum(E446:E461); others at AF445:AXC445, AXD445:BPO445, E466:AWJ466, AWK466:AXC466, AXD466:BPO466, E491:AHD491 +21
ƒ345–357 13 similar groups (13189 cells; same shape, refs/constants differ) e.g. VU457:WX457 → =('Item Sales (POS)'!VU24+'Item Sales (POS)'!VU310+'Item Sales (POS)'!VU258)*$D457; others at E462:BNF462, BNG462:BPO462, JT471:BNF473, BNG471:BPO473, JT474:BNF476, BNG474:BPO476 +6
ƒ358–380 23 similar groups (6580 cells; same shape, refs/constants differ) e.g. AHE508:BNF508 → =('Item Sales (POS)'!AHE76+'Item Sales (POS)'!AHE84+'Item Sales (POS)'!AHE231+'Item Sales (POS)'!AHE264)*$D508; others at BNG508:BPO508, QT566:TE568, TG566:VK568, VT569:ZH570, ZI570:AZU570, BFN570:BNF570 +16
ƒ381–384 4 similar groups (704 cells; same shape, refs/constants differ) e.g. BCB527:BNF527 → ='Item Sales (POS)'!BCB368+'Item Sales (POS)'!BCB373*$D527; others at BNG527:BPO527, BCB528:BNF528, BNG528:BPO528
ƒ385–397 13 similar groups (5801 cells; same shape, refs/constants differ) e.g. TF566:TF568 ↓ =('Item Sales (POS)'!TF66+'Item Sales (POS)'!TF218+'Item Sales (POS)'!TF251+'Item Sales (POS)'!TF273+'Item Sales (POS)'!TF349)*$D566; others at VL566:AZU568, BFN566:BNF568, BNG566:BPO568, ZI569:AZU569, BFN569:BNF569, BNG569:BPO569 +6
ƒ398 AMU574 =('Item Sales (POS)'!AMU76+'Item Sales (POS)'!AMU84+'Item Sales (POS)'!AMU231+'Item Sales (POS)'!AMU264)*$D574+50
ƒ399 AWK616:BGG616 → =('Item Sales (POS)'!AWK401)+('Item Sales (POS)'!AWK428)*$D612
ƒ400 E624:BPO624 → =iferror(sumifs(E$154:E$188,$B$154:$B$188,$B624)+E627+(E626/2),0)
ƒ401 E625:BPO625 → =iferror(sumifs(E$154:E$188,$B$154:$B$188,$B625)+E628+(E626/2),0)
ƒ402 E626:BPO626 → =sumifs(E$154:E$190,$B$154:$B$190,$B626)+E629
ƒ403–406 4 similar groups (3558 cells; same shape, refs/constants differ) e.g. E627:BNF627 → =sumifs(E$219:E$238,$B$219:$B$238,$B627) * ('Item Sales (POS)'!E327/('Item Sales (POS)'!E327+'Item Sales (POS)'!E64)) [26 errors]; others at BNG627:BPO627, E628:BNF628, BNG628:BPO628
◆ formatting
bg#FFF2CC (261055 cells): A1:A82, A87:A592, A594:A716, B6:BPO9 +12
bg#EFEFEF (176418 cells): B199:BPO218, B357:BPO435
bg#F9CB9C (124740 cells): B10:BPO13, B40:BPO42, B150:BPO151, B180:BPO181 +6
bg#F4CCCC (69499 cells): A593:BPO593, B1:C4, B18:BPO21, B51:BPO63 +7
bg#D9EAD3 (55242 cells): B34:BPO36, B64:BPO65, B160:BPO161, B174:BPO176 +5
bg#CFE2F3 (32076 cells): B14:BPO17, B68:BPO78, B168:BPO170
bg#FCE5CD (24952 cells): A83:BPO86, B22:BPO25, B154:BPO156, B260:BPO261 +1
bg#D9D2E9 (16038 cells): B26:BPO29, B171:BPO173, B185:BPO186
+7 more styles
hidden rows: 126-153, 206-216, 265-271, 314-333, 358-434
hidden cols: F-KX, KZ-YY, ZA-ANA, ANC-BBB

## 6. Item Sales (POS) · A1:BNX812 · 967k cells · frozen 5r4c
read by: Monthly Summary (Usage), Monthly Summary (item sales), Item Sales (Cal)
▸ A1:BNX812 table 812×1740 (966760 cells), no header row
  A -: ƒ4; num 1 … 808 (808 filled)
  C -: text 656 distinct e.g. "ข้าวสาหร่าย", "ปูอัดฉีก (30)", "มะม่วง" (707 filled)
  D -: text 3 distinct e.g. "แทนโดย", "ไม่รับ", "ไม่รับท้อปปิ้ง" (3 filled)
  E…AR (40 alike): date 1-Mar-22; num 0 … 2022 (443 filled)
  AS -: date 10-Apr-22; num 0 … 2022; text "10 April" (448 filled)
  AT -: date 11-Apr-22; num 0 … 44662 (448 filled)
  AU -: date 12-Apr-22; num 0 … 44663; text "Item Sales Report" (449 filled)
  AV…BF (11 alike): date 13-Apr-22; num 0 … 2022 (446 filled)
  BG -: date 24-Apr-22; num 0 … 2022; text "24 April" (448 filled)
  BH…EE (76 alike): date 25-Apr-22; num 0 … 2022 (446 filled)
  EF -: date 10-Jul-22; num 0 … 2022; text "10 July" (456 filled)
  EG…BNX (1604 alike): date 11-Jul-22; num 0 … 2022 (454 filled)
  sample (A…P of 1740):
  1|  |  |  |  | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022
  2|  |  |  |  | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3
  3|  |  | วันที่ |  | 1-Mar-22 | 2-Mar-22 | 3-Mar-22 | 4-Mar-22 | 5-Mar-22 | 6-Mar-22 | 7-Mar-22 | 8-Mar-22 | 9-Mar-22 | 10-Mar-22 | 11-Mar-22 | 12-Mar-22
  812| 808 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
ƒ formulas (11k cells, 11 groups, 6 families)
ƒ1 E1:BNX1 → =year(E3)
ƒ2 E2:BNX2 → =month(E3)
ƒ3 F3:BNX3 → =E3+1
ƒ4 A6:A812 ↓ =A5+1
ƒ6–8 E713:BHI714 ▦ =E495+E499+E503+E507+E511+E515+E519+E523+E527+E531+E549 — same formula in 3 ranges (3468 cells): BHK713:BJK714, BJM713:BNX714; breaks at BHJ713:BHJ714, BJL713:BJL714 = 4 hardcoded values
ƒ9–11 E715:BHI715 → =E534 — same formula in 3 ranges (1734 cells): BHK715:BJK715, BJM715:BNX715; breaks at BHJ715, BJL715 = 2 hardcoded values
typed arithmetic: 1 cells hold ref-free formulas (hand-entered sums) e.g. `=1+1` in columns CC
◆ formatting
bg#FFF2CC (322194 cells): A6:A58, A61:A362, A388:A401, A439:A812 +37
bg#EFEFEF (269177 cells): B383:ANF400, B577:ANF710, B713:D715, E713:E812 +10
bg#D9EAD3 (145833 cells): A363:ANF367, B30:ANF32, B206:ANF207, B209:ANF243 +30
bg#FCE5CD (126781 cells): A368:A387, A402:A438, B45:ANF48, B51:ANF58 +24
bg#F9CB9C (102424 cells): B10:ANF13, B36:ANF38, B321:ANF323, B445:ANF493 +12
bg#D9D2E9 (69440 cells): B22:ANF25, B309:ANF311, B330:ANF362, ANH22:ANM25 +8
bg#F4CCCC (55557 cells): A1:A5, A59:ANF60, B1:C4, B18:ANF21 +26
bg#CFE2F3 (50344 cells): B14:ANF17, B99:ANF120, B312:ANF314, ANH14:ANM17 +8
+20 more styles
hidden cols: F-KX, KZ-YY, ZA-ANA, ANC-BBB

## 7. Item Sales (Usage) · A1:M2000 · 24k cells · frozen 2r0c
reads: Item Sales (Cal) (10g)
read by: Salmon, Tuna, Chicken, Chicken BB, Avocado, Mango, MahachanokMango, Pomelo, Edamame, Tamago, Ikura
▸ A1:M2000 table 2000×13 (24251 cells), header row 2
  A "[1] Item": date Tue, Mar 1, 2022 … Wed, Aug 18, 2027; text "Date" (1998 filled)
  B "[2] ": ƒ2; num 1 … 12; text "Month" (1998 filled)
  C "[3] ": ƒ3; num 2022 … 2027; text "Year" (1998 filled)
  D "[4] Salmon": ƒ4; num 0 … 3705 (1997 filled)
  E "[5] Tuna": ƒ5; num -30 … 2005 (1997 filled)
  F "[6] Mango": ƒ6; num 0  … 1,364  (1997 filled)
  G "[7] Avocado": ƒ7; num 0 … 1596 (1997 filled)
  H "[8] Pomelo": ƒ8; num 0  … 920  (1997 filled)
  I "[9] Tamago": ƒ9; num 0  … 1,534  (1997 filled)
  J "[10] Edamame": ƒ10; num 0  … 1,188  (1997 filled)
  K "[11] Ikura": ƒ11; num 0 … 263 (1997 filled)
  L "[12] Chicken": ƒ12; num 0 … 1930 (1540 filled)
  M "[13] ChickenBB": ƒ13; num 0 … 1781 (717 filled)
  sample (A…M):
  3| Date | Month | Year |  |  |  |  |  |  |  |  |  | 
  4| Tue, Mar 1, 2022 | 3 | 2022 | 1680 | 630 | 836  | 360 | 340  | 720  | 638  | 118 |  | 
  5| Wed, Mar 2, 2022 | 3 | 2022 | 1860 | 420 | 924  | 576 | 680  | 384  | 418  | 60 |  | 
  2000| Wed, Aug 18, 2027 | 8 | 2027 | 0 | 0 | 0  | 0 | 0  | 0  | 0  | 0 | 0 | 0
ƒ formulas (22k cells, 13 groups, 4 families)
ƒ1 B1:M1 → =A1+1
ƒ2 B4:B2000 ↓ =month(A4)
ƒ3 C4:C2000 ↓ =year(A4)
ƒ4–13 10 similar groups (18233 cells; same shape, refs/constants differ) e.g. D4:D2000 ↓ =hlookup($A4,'Item Sales (Cal)'!$3:627,'Item Sales (Cal)'!A$442,1); others at E4:E2000, F4:F2000, G4:G2000, H4:H2000, I4:I2000, J4:J2000 +3
◆ formatting
bg#FFF2CC (5994 cells): F3:F2000, H3:H2000, L3:L2000
bg#D9EAD3 (5994 cells): G3:G2000, J3:J2000, M3:M2000
bg#F9CB9C (1998 cells): D3:D2000
bg#F4CCCC (1998 cells): E3:E2000
bg#FFE599 (1998 cells): I3:I2000
bg#FCE5CD (1998 cells): K3:K2000
italic (12 cells): A1:L1
bold (3 cells): A2:C2
+8 more styles

## 8. Inventory Summary · A1:I34 · 117 cells
reads: Fish record (6g), Salmon (3g), Tuna (2g), Avocado (3g), Mango (3g), MahachanokMango (1g), Pomelo (2g), Edamame (5g), Tamago (5g), Eel (2g), Ikura (2g), Others (42g)
▸ A1:D2 (5 cells)
  1: A=Date B=ƒ1→10/23/2026
  2: A=EMQ C=คงเหลือ Raw materials  … D=คงเหลือ (Beginning Bala…
▸ G1:I5 (9 cells)
  1: G=ƒ2→10/22/2026
  2: G=Waste : Production H=Expired
  3: H=Production I=Ready to use
  4: H=ƒ4→
  5: G=ƒ6→171 H=ƒ7→0 I=ƒ8→12
▸ A4:D34 table 31×4 (85 cells), no header row
  A -: text 31 distinct e.g. "Avocado", "Edamame", "Eel" (31 filled)
  C -: ƒ3,ƒ5,ƒ9 +28; num 0 … 9690 (31 filled)
  D -: ƒ22,ƒ24,ƒ29 +20; num 0 … 2040 (23 filled)
  sample (A…D):
  4| Salmon Proton |  | 9690 | 
  5| Salmon |  | 1505 | 
  6| Tuna (Frozen) |  | 4900 | 
  34| ข้าว |  | 8 | 
▸ G7:I8 (5 cells)
  7: G=ƒ11→0 H=ƒ12→0 I=ƒ13→0
  8: G=ƒ15→460 H=ƒ16→167
▸ G10:H10 (2 cells)
  10: G=ƒ19→ H=ƒ20→
▸ G12:I13 (6 cells)
  12: G=ƒ25→ H=ƒ26→ I=ƒ27→
  13: G=ƒ30→ H=ƒ31→ I=ƒ32→
▸ I20 (1 cells)
  20: I=ƒ47→
▸ G22 (1 cells)
  22: G=ƒ52→
▸ I25 (1 cells)
  25: I=ƒ59→
▸ G26 (1 cells)
  26: G=ƒ62→324
▸ I28 (1 cells)
  28: I=ƒ67→100
ƒ formulas (78 cells, 78 groups, 3 families)
ƒ1 B1 =today() [volatile]
ƒ2 G1 =today()-1 [volatile]
ƒ3–78 76 similar groups (76 cells; same shape, refs/constants differ) e.g. C4  =INDEX(Salmon!$V:$V,match($B$1,Salmon!$A:$A,0),); others at H4, C5, G5, H5, I5, C6 +69
◆ formatting
bg#FFF2CC (47 cells): C16:C34, D16:D33, G8:H8, G12:I12 +5
bg#D9EAD3 (5 cells): G10:H10, G13:I13
bg#FCE5CD (3 cells): G5:I5
bg#F4CCCC (3 cells): G7:I7
bg#FFFF00 (2 cells): B1, G1
bold underline (1 cells): A2
merged: A2:A3, C2:C3, D2:D3, E2:E3, G2:G3 +1

## 9. Monthly Inventory Summary · A1:AE35 · 941 cells
reads: Salmon (14g), Tuna (14g), Avocado (7g), Mango (7g), MahachanokMango (7g), Pomelo (14g), Edamame (14g), Tamago (14g), Eel (14g), Ikura (14g), Others (259g)
▸ A1:AE33 table 33×31 (933 cells), header row 2
  A "รหัส": num 1001 … 3002 (26 filled)
  B "EMQ": text 31 distinct e.g. "Avocado", "Edamame", "Eel" (31 filled)
  C "ราคาต่อกรัม": ƒ414,ƒ415,ƒ416 +13; num 0.02 … 180.00 fmt 0.00 (26 filled)
  D "คงเหลือ ⏎Raw material": ƒ1,ƒ8,ƒ15 +28; num 0 … 6000 (31 filled)
  E "คงเหลือ ⏎Ready to use": ƒ51,ƒ65,ƒ79 +20; num 0 … 1177 (23 filled)
  F "คงเหลือ⏎Total": ƒ379,ƒ407,ƒ386; num 0 … 6874 (31 filled)
  G "[31-Dec-2022] มูลค่า": ƒ400; num 0.00 … 5,543.00 fmt #,##0.00;(#,##0.00) (31 filled)
  H "คงเหลือ ⏎Raw material": ƒ2,ƒ9,ƒ16 +28; num 0 … 5330 (31 filled)
  I "คงเหลือ ⏎Ready to use": ƒ53,ƒ67,ƒ81 +20; num 0 … 1449 (23 filled)
  J "คงเหลือ⏎Total": ƒ380,ƒ408,ƒ387; num 0 … 5330 (31 filled)
  K "[31-Jan-2023] มูลค่า": ƒ401; num 0.00 … 4,950.00 fmt #,##0.00;(#,##0.00) (31 filled)
  L "คงเหลือ ⏎Raw material": ƒ3,ƒ10,ƒ17 +28; num 0 … 5000 (31 filled)
  M "คงเหลือ ⏎Ready to use": ƒ55,ƒ69,ƒ83 +20; num 0 … 1170 (23 filled)
  N "คงเหลือ⏎Total": ƒ381,ƒ409,ƒ388; num 0 … 5213 (31 filled)
  O "[28-Feb-2023] มูลค่า": ƒ402; num 0 … 4998.75 (31 filled)
  P "คงเหลือ ⏎Raw material": ƒ4,ƒ11,ƒ18 +28; num 0 … 8500 (31 filled)
  Q "คงเหลือ ⏎Ready to use": ƒ57,ƒ71,ƒ85 +20; num 0 … 1889 (23 filled)
  R "คงเหลือ⏎Total": ƒ382,ƒ410,ƒ389; num 0 … 8627 (31 filled)
  S "[31-Mar-2023] มูลค่า": ƒ403; num 0 … 5823.75 (31 filled)
  T "คงเหลือ ⏎Raw material": ƒ5,ƒ12,ƒ19 +28; num 0 … 4630 (31 filled)
  U "คงเหลือ ⏎Ready to use": ƒ59,ƒ73,ƒ87 +20; num 0 … 1273 (23 filled)
  V "คงเหลือ⏎Total": ƒ383,ƒ411,ƒ390; num 0 … 4630 (31 filled)
  W "[30-Apr-2023] มูลค่า": ƒ404; num 0 … 4553 (31 filled)
  X "คงเหลือ ⏎Raw material": ƒ6,ƒ13,ƒ20 +28; num 0 … 8310 (31 filled)
  Y "คงเหลือ ⏎Ready to use": ƒ61,ƒ75,ƒ89 +20; num 0 … 1480 (23 filled)
  Z "คงเหลือ⏎Total": ƒ384,ƒ412,ƒ391; num 0 … 8310 (31 filled)
  AA "[31-May-2023] มูลค่า": ƒ405; num 0 … 5152.2 (31 filled)
  AB "คงเหลือ ⏎Raw material": ƒ7,ƒ14,ƒ21 +28; num 0 … 6450 (31 filled)
  AC "คงเหลือ ⏎Ready to use": ƒ63,ƒ77,ƒ91 +20; num 0 … 1661 (23 filled)
  AD "คงเหลือ⏎Total": ƒ385,ƒ413,ƒ392; num 0 … 6450 (31 filled)
  AE "[30-Jun-2023] มูลค่า": ƒ406; num 0 … 4687.5 (31 filled)
  sample (A…P of 31):
  3| 1002 | Salmon Proton | 0.62 | 5060 |  | 5060 | 3,137.20 | 5330 |  | 5330 | 3,304.60 | 3320 |  | 3320 | 2058.4 | 6490
  4| 1001 | Salmon | 0.60 | 4910 |  | 4910 | 2,946.00 | 2280 |  | 2280 | 1,368.00 | 580 |  | 580 | 348 | 3190
  5| 1101 | Tuna (Frozen) | 0.50 | 2520 |  | 2520 | 1,260.00 | 0 |  | 0 | 0.00 | 0 |  | 0 | 0 | 1780
  33| 3002 | ข้าว | 180.00 | 10 |  | 10 | 1,800.00 | 4 |  | 4 | 720.00 | 4 |  | 4 | 720 | 12
▸ B35 (1 cells)
  35: B=Total
▸ G35 (1 cells)
  35: G=ƒ393→25,913.08
▸ K35 (1 cells)
  35: K=ƒ394→19,271.59
▸ O35 (1 cells)
  35: O=ƒ395→17,169.42
▸ S35 (1 cells)
  35: S=ƒ396→26,856.88
▸ W35 (1 cells)
  35: W=ƒ397→18,322.90
▸ AA35 (1 cells)
  35: AA=ƒ398→24,457.52
▸ AE35 (1 cells)
  35: AE=ƒ399→25,108.38
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

## 10. Fish record · A1:W2000 · 30k cells · frozen 2r1c
read by: Monthly Summary (Usage), Inventory Summary, Salmon, Tuna
▸ A1:W2000 table 2000×23 (29513 cells), header row 2
  A "[1] Date": date Tue 01-03-2022  … Thu 19-08-2027  (1998 filled)
  B "[2] Month": ƒ2; num 1 … 12 (1998 filled)
  C "[3] Year": ƒ3; num 2022 … 2027 (1998 filled)
  D "[4] Salmon Total Weight": num 0 … 4453 (1998 filled)
  E "[5] Salmon Waste": num 0 … 2183 (1998 filled)
  F "[6] Salmon Waste (Closed)": num 0 … 720 (1998 filled)
  G "[7] Salmon Waste (expired)": num 0 (1998 filled)
  H "[8] Salmon Net Weight": ƒ5; num 0 … 3875 (1998 filled)
  I "[9] Salmon Yield": ƒ7; num 48.51% … 97.81% fmt 0.00% (1998 filled)
  J "[10] Tuna Total Weight": num 0 … 2146 (1998 filled)
  K "[11] Tuna Waste": num 0 … 200 (1998 filled)
  L "[12] Tuna Waste (Closed)": num 0 … 283 (1496 filled)
  M "[13] Tuna Waste (expired)": num 0 (1998 filled)
  N "[14] Tuna Net Weight": ƒ6; num 0 … 2146 (1998 filled)
  O "[15] Tuna Yield": ƒ8; num 75.63% … 100.00% fmt 0.00% (1998 filled)
  P "[16] TCM": empty
  Q "[17] $/g": empty
  R "[18] Discount": empty
  S "[19] Refund": empty
  T "[20] Non Poke Revenue": empty
  U "[21] $ excl. non poke": empty
  V "[22] $/g excl. non poke": empty
  W "Total Waste (closed)": empty
  sample (A…P of 23):
  3| Tue 01-03-2022  | 3 | 2022 | 1935 | 149 | 140 | 0 | 1786 | 92.30% | 651 | 0 | 31 | 0 | 651 | 100.00% | 
  4| Wed 02-03-2022  | 3 | 2022 | 2201 | 196 | 206 | 0 | 2005 | 91.09% | 548 | 0 | 106 | 0 | 548 | 100.00% | 
  5| Thu 03-03-2022  | 3 | 2022 | 1814 | 160 | 16 | 0 | 1654 | 91.18% | 805 | 0 |  | 0 | 805 | 100.00% | 
  2000| Thu 19-08-2027  | 8 | 2027 | 0 | 0 | 0 | 0 | 0 |  | 0 | 0 |  | 0 | 0 |  | 
ƒ formulas (12k cells, 9 groups, 7 families)
ƒ1 B1:V1 → =A1+1
ƒ2 B3:B2000 ↓ =month(A3)
ƒ3 C3:C2000 ↓ =Year(A3)
ƒ4 D3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=871256527","Fish record summary!D3:g2000") [external] [array/spills]
ƒ5–6 H3:H2000 ↓ =D3-E3 — same formula in 2 ranges (3996 cells): N3:N2000; breaks at I3:M2000 = 7489 hardcoded values, 1999 different formulas, 502 blank
ƒ7–8 I3:I2000 ↓ =IF(D3>0, H3/D3, ) — same formula in 2 ranges (3996 cells): O3:O2000; breaks at J3:N2000 = 7489 hardcoded values, 1999 different formulas, 502 blank
ƒ9 J3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=871256527","Fish record summary!j3:m2000") [external] [array/spills]
◆ formatting
fg#0000FF (9990 cells): A3:A2000, H3:I2000, N3:O2000
italic (22 cells): A1:V1
bg#D9D9D9 bold underline (7 cells): P2:V2
bg#FCE5CD bold underline (6 cells): D2:I2
bg#F4CCCC bold underline (6 cells): J2:O2
bold underline (4 cells): A2:C2, W2
bg#FCE5CD (2 cells): D3, J3

## 11. Salmon · A1:V2001 · 37k cells · frozen 3r3c
reads: Sales (2g), Item Sales (Usage) (1g), Fish record (5g)
read by: Monthly Summary (Usage), Inventory Summary, Monthly Inventory Summary
▸ A1:V2001 table 2001×22 (36842 cells), header row 3
  A "[1 / รายการ] Date": date Tue, Mar 1, 2022 … Thu, Aug 19, 2027 (1998 filled)
  B "[2] Month": ƒ4; num 1 … 12 (1998 filled)
  C "[3] Year": ƒ5; num 2022 … 2027 (1998 filled)
  D "[Salmon (กรัม)] Salmon (กรัม/1000บาท)": ƒ6,ƒ7; num 55.12 … 393.03 fmt 0.00; 304 errors (1998 filled)
  E "[4 / 1/1] คงเหลือ": num 0 … 7630 (1698 filled)
  F "[5] ของเข้า(+)": num 660 … 8390 (1420 filled)
  G "[6] ของออก(-)": num 650 … 2120 (8 filled)
  H "[7] ไม่ได้คุณภาพ(-)": H1040=79 H1041=29
  I "[8] หมดอายุ(-)": I142=2000 I150=1150
  J "[9] ใช้ไป (ใบของสด)": num 0  … 4,560  (1998 filled)
  K "[10] ใช้ไป(ใบจดปลา) ก่อน waste": ƒ9; num 0 … 4453 (1998 filled)
  L "[11] (ใบนับของสด-ใบจดปลา)": ƒ15; num (1,650) … 3,000  fmt #,##0_);[Red](#,##0) (1998 filled)
  M "[12] % diff": ƒ16; num -44.48% … 222.22% fmt 0.00%; 304 errors (1998 filled)
  N "[13] Check ใบของสด": ƒ18; enum Correct(1535) Re-check(159); 304 errors (1998 filled)
  O "[14] ใช้ไป(ใบจดปลา) หลัง waste": ƒ10; num 0 … 3875 (1998 filled)
  P "[15] waste ปิดร้าน (-)": ƒ11,ƒ14; num 0 … 720 (1998 filled)
  Q "[16] waste หมดอายุ (-)": ƒ12; num 0 (1998 filled)
  R "[17] item sales": ƒ13; num 0 … 3705 (1998 filled)
  S "[18 ] ใบจดปลา-Item sales": ƒ20; num (277) … 429  fmt #,##0_);[Red](#,##0) (1998 filled)
  T "[19 ] % diff": ƒ17; num -15.59% … 27.24% fmt 0.00%; 304 errors (1998 filled)
  U "[20 ] Check item sales": ƒ19; enum Correct(1660) Re-check(34); 304 errors (1998 filled)
  V "[21 ] คงเหลือ Proton": num 0 … 35500 (1698 filled)
  sample (A…P of 22):
  4| Tue, Mar 1, 2022 | 3 | 2022 | 130.11 | 3830 | 3120 |  |  |  | 2,020  | 1935 | 85  | 4.39% | Correct | 1786 | 140
  5| Wed, Mar 2, 2022 | 3 | 2022 | 146.71 | 4930 |  |  |  |  | 2,480  | 2201 | 279  | 12.68% | Re-check | 2005 | 206
  6| Thu, Mar 3, 2022 | 3 | 2022 | 125.32 | 2450 | 1484 |  |  |  | 1,964  | 1814 | 150  | 8.27% | Correct | 1654 | 16
  2001| Thu, Aug 19, 2027 | 8 | 2027 | #DIV/0! Function DIVIDE… |  |  |  |  |  | 0  | 0 | 0  | #DIV/0! Function DIVIDE… | #DIV/0! Function DIVIDE… | 0 | 
ƒ formulas (28k cells, 21 groups, 11 families)
ƒ1–3 3 similar groups (20 cells; same shape, refs/constants differ) e.g. B1:C1 → =A1+1; others at E1, F1:V1
ƒ4 B4:B2001 ↓ =month(A4)
ƒ5 C4:C2001 ↓ =year(A4)
ƒ6 D4:D1913 ↓ =(K4/Sales!$G4)*1000 [216 errors]
ƒ7 D1914:D2001 ↓ =(K1914/Sales!$G2501)*1000 [88 errors]
ƒ8 E4 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1444348670","1/1!c3:h2000") [external] [array/spills]
ƒ9–14 6 similar groups (9990 cells; same shape, refs/constants differ) e.g. K4:K2001 ↓ ='Fish record'!D3; others at O4:O2001, P4:P1728, Q4:Q2001, R4:R2001, P1729:P2001
ƒ15 L4:L2001 ↓ =J4-K4
ƒ16–17 M4:M2001 ↓ =L4/K4 [304 errors] — same formula in 2 ranges (3996 cells): T4:T2001; breaks at N4:S2001 = 11988 different formulas
ƒ18 N4:N2001 ↓ =if(and(M4<10%,M4>-10%),"Correct","Re-check") [304 errors]
ƒ19 U4:U2001 ↓ =if(and(T4<13%,T4>-5%),"Correct","Re-check") [304 errors]
ƒ20 S4:S2001 ↓ =O4-P4-Q4-R4
ƒ21 V4 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1444348670","1/1!k3:k2000") [external] [array/spills]
◐ conditional formats
N2:N2001, U4:U2001: TEXT_EQ Correct → bg#B7E1CD
N2:N2001, U4:U2001: TEXT_EQ Re-check → bg#EA9999
A1:A2001: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#EFEFEF (17982 cells): L4:Q2001, S4:U2001
bg#FCE5CD (17167 cells): D4:D2001, E5:E2001, F4:K2001, V2 +1
bg#D9EAD3 fg#0000FF (1998 cells): R4:R2001
italic (22 cells): A1:V1
bg#EFEFEF bold (18 cells): L2:Q3, S2:U3
bg#FCE5CD bold (17 cells): D2:K3, V3
bold (3 cells): A2:C2
bold underline (3 cells): A3:C3
+3 more styles
hidden rows: 5-309, 311-674, 676-1405

## 12. Tuna · A1:V2000 · 37k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g), Fish record (104g)
read by: Monthly Summary (Usage), Inventory Summary, Monthly Inventory Summary
same layout as "Salmon" (74% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Tuna"`
blocks: A1:V2000 (36737 cells)
ƒ 28k cells in 118 groups; 3 families not in "Salmon":
ƒ3 D3:D1912 ↓ =K3/Sales!$G4*1000 [216 errors]
ƒ4 D1913:D2000 ↓ =K1913/Sales!$G2501*1000 [88 errors]
ƒ117 V3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1444348670","1/2!c3:c2000") [external] [array/spills]
ƒ118 E12 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1444348670","1/1!s12:x2000") [external] [array/spills]
labels not in "Salmon": B:"`" D:"Tuna (กรัม)" D:"Tuna (กรัม/1000บาท)" V:"คงเหลือ Tuna (Frozen)"

## 13. Chicken · A1:V2043 · 29k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage)
▸ A1:V2043 table 2043×22 (28837 cells), header row 2
  A "Date": date Thu, Jun 1, 2023 … Sun, Dec 31, 2028 (2041 filled)
  B "Month": ƒ1; num 1 … 12 (2041 filled)
  C "Year": ƒ2; num 2023 … 2028 (2041 filled)
  D "[Chicken (กรัม)] Chicken (กรัม/1000บาท)": ƒ3,ƒ4; num 0.00 … 489.86 fmt 0.00; 347 errors (2041 filled)
  E "[9/2] คงเหลือ": num 0 … 34000 (1241 filled)
  F "ของเข้า(+)": num 100 … 30000 (154 filled)
  G "ของออก(-)": num 500 … 10000 (12 filled)
  H "ไม่ได้คุณภาพ(-)": H839=143 H901=94
  I "หมดอายุ(-)": num 68 … 188 (28 filled)
  J "ใช้ไป (ใบของสด)": num 0 … 2650 (1541 filled)
  K "เตรียมได้": num 200 … 2650 (491 filled)
  L "[5/3] ใช้ไป⏎(ใบเตรียมของ)": num 0 … 2035 (1541 filled)
  M "ไม่ได้คุณภาพ(-)": ƒ8 (1 filled)
  N "หมดอายุ(-)": num 20 … 900 (108 filled)
  O "เปิดร้าน (ใบเตรียมของ)": num 0 … 2640; text "0" (1241 filled)
  P "(ใบนับของสด-เตรียมได้)": ƒ11; num (1,088) … 1,000  fmt #,##0_);[Red](#,##0) (2041 filled)
  Q "[95-115%] % diff": ƒ13; num 0.00% … 143.00% fmt 0.00% (2041 filled)
  R "Check ใบของสด": ƒ14; enum Correct(1982) Re-check(59) (2041 filled)
  S "item sales": ƒ15; num 0 … 1930 (2041 filled)
  T "ใบเตรียมของ-Item sales": ƒ12; num (1,550) … 1,280  fmt #,##0_);[Red](#,##0) (2041 filled)
  U "% diff": ƒ16; num -100.00% … 175.51% fmt 0.00%; 993 errors (2041 filled)
  V "Check item sales": ƒ17; enum Correct(960) Re-check(88); 993 errors (2041 filled)
  sample (A…P of 22):
  3| Thu, Jun 1, 2023 | 6 | 2023 | 20.17 |  | 8000 |  |  |  | 1000 | 1000 | 300 |  |  |  | 0 
  4| Fri, Jun 2, 2023 | 6 | 2023 | 20.00 | 7000 |  |  |  |  | 0 |  | 300 |  |  | 700 | 0 
  5| Sat, Jun 3, 2023 | 6 | 2023 | 13.82 | 7000 |  |  |  |  | 0 |  | 200 |  |  | 400 | 0 
  2043| Sun, Dec 31, 2028 | 12 | 2028 | #DIV/0! Function DIVIDE… |  |  |  |  |  |  |  |  |  |  |  | 0 
ƒ formulas (20k cells, 17 groups, 15 families)
ƒ1 B3:B2043 ↓ =month(A3)
ƒ2 C3:C2043 ↓ =year(A3)
ƒ3 D3:D1912 ↓ =L3/Sales!$G4*1000 [216 errors]
ƒ4 D1913:D2043 ↓ =L1913/Sales!$G2501*1000 [131 errors]
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1444348670","9/2!K460:P2000") [external] [array/spills]
ƒ6 K3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1868553393","5/3!E460:E2000") [external] [array/spills]
ƒ7 L3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1868553393","5/3!J460:J2000") [external] [array/spills]
ƒ8 M3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1868553393","5/3!G460:G2000") [external] [array/spills]
ƒ9 N3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1868553393","5/3!H460:H2000") [external] [array/spills]
ƒ10 O3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1868553393","5/3!C460:C2000") [external] [array/spills]
ƒ11 P3:P2043 ↓ =J3-K3
ƒ12 T3:T2043 ↓ =L3-S3
ƒ13 Q3:Q2043 ↓ =iferror(K3/J3,0)
ƒ14 R3:R2043 ↓ =if(Or(and(Q3<115%,Q3>95%),and(P3=0,Q3=0%)),"Correct","Re-check")
ƒ15 S3:S2043 ↓ ='Item Sales (Usage)'!L461
ƒ16 U3:U2043 ↓ =T3/S3 [993 errors]
ƒ17 V3:V2043 ↓ =if(and(U3<13%,U3>-5%),"Correct","Re-check") [993 errors]
◐ conditional formats
R1:R2043, V3:V2043: TEXT_EQ Correct → bg#B7E1CD
R1:R2043, V3:V2043: TEXT_EQ Re-check → bg#EA9999
A1:A2043: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (24486 cells): D3:D2043, E4:E2043, F3:J2043, K4:O2043
bg#EFEFEF (12246 cells): P3:R2043, T3:V2043
bg#D9EAD3 fg#0000FF (2041 cells): S3:S2043
bg#FCE5CD bold (24 cells): D1:O2
bg#EFEFEF bold (12 cells): P1:R2, T1:V2
bg#F9CB9C (6 cells): E3, K3:O3
bold (3 cells): A1:C1
bold underline (3 cells): A2:C2
+2 more styles
hidden rows: 4-216, 218-947

## 14. Chicken BB · A1:V2043 · 26k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage)
same layout as "Chicken" (91% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Chicken BB"`
blocks: A1:V2043 (25885 cells)
ƒ 20k cells in 17 groups; 6 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1444348670","9/2!C460:H2000") [external] [array/spills]
ƒ6 K3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1868553393","5/3!N460:N2000") [external] [array/spills]
ƒ7 L3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1868553393","5/3!S460:S2000") [external] [array/spills]
ƒ8 M3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1868553393","5/3!P460:P2000") [external] [array/spills]
ƒ9 N3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1868553393","5/3!Q460:Q2000") [external] [array/spills]
ƒ10 O3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1868553393","5/3!L460:L2000") [external] [array/spills]
labels not in "Chicken": A:"รายการ" D:"Chicken BB (กรัม)"

## 15. Avocado · A1:N2000 · 21k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage), Inventory Summary, Monthly Inventory Summary
▸ A1:N2000 table 2000×14 (21313 cells), header row 2
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Thu, Aug 19, 2027 (1998 filled)
  B "Month": ƒ1; num 1 … 12 (1998 filled)
  C "Year": ƒ2; num 2022 … 2027 (1998 filled)
  D "[Avocado (กรัม)] อโวคาโด (กรัม/1000บาท)": ƒ3,ƒ4; num 28.25 … 185.61 fmt 0.00; 304 errors (1998 filled)
  E "[2/1] คงเหลือ": num 0 … 9335 (1698 filled)
  F "ของเข้า(+)": num 630 … 11045 (780 filled)
  G "ของออก(-)": num 1690 … 3985 (5 filled)
  H "ไม่ได้คุณภาพ(-)": num 16 … 7700 (827 filled)
  I "หมดอายุ(-)": num 100 … 150 (3 filled)
  J "ใช้ไป (ใบของสด)": num 0 … 6890 (1998 filled)
  K "item sales": ƒ6; num 0 … 1596 (1998 filled)
  L "ใช่ไปใบของสด-Item sales": ƒ7; num -28 … 6890 (1998 filled)
  M "[40%] % Yield": ƒ8; num 0.00% … 103.50% fmt 0.00%; 303 errors (1998 filled)
  N "Check item sales": ƒ9; enum Correct(1268) Re-check(427); 303 errors (1998 filled)
  sample (A…N):
  3| Tue, Mar 1, 2022 | 3 | 2022 | 50.77 | 3390 | 3580 |  |  |  | 755 | 360 | 395 | 47.68% | Correct
  4| Wed, Mar 2, 2022 | 3 | 2022 | 82.99 | 6215 |  |  |  |  | 1245 | 576 | 669 | 46.27% | Correct
  5| Thu, Mar 3, 2022 | 3 | 2022 | 96.03 | 4970 | 910 |  |  |  | 1390 | 456 | 934 | 32.81% | Re-check
  2000| Thu, Aug 19, 2027 | 8 | 2027 | #DIV/0! Function DIVIDE… |  |  |  |  |  | 0 |  | 0 | #DIV/0! Function DIVIDE… | #DIV/0! Function DIVIDE…
ƒ formulas (14k cells, 9 groups, 8 families)
ƒ1 B3:B2000 ↓ =month(A3)
ƒ2 C3:C2000 ↓ =year(A3)
ƒ3 D3:D1912 ↓ =J3/Sales!$G4*1000 [216 errors]
ƒ4 D1913:D2000 ↓ =J1913/Sales!$G2501*1000 [88 errors]
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","2/1!c3:h2000") [external] [array/spills]
ƒ6 K3:K2000 ↓ ='Item Sales (Usage)'!G4
ƒ7 L3:L2000 ↓ =J3-K3
ƒ8 M3:M2000 ↓ =K3/J3 [303 errors]
ƒ9 N3:N2000 ↓ =if(and(M3<100%,M3>40%),"Correct","Re-check") [303 errors]
◐ conditional formats
N3:N2000: TEXT_EQ Correct → bg#B7E1CD
N3:N2000: TEXT_EQ Re-check → bg#EA9999
A1:A2000: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (13985 cells): D3:D2000, E4:E2000, F3:J2000
bg#EFEFEF (11988 cells): A3:C2000, L3:N2000
bg#D9EAD3 fg#0000FF (1998 cells): K3:K2000
bg#FCE5CD bold (14 cells): D1:J2
bg#EFEFEF bold (9 cells): A1:C1, L1:N2
bg#EFEFEF bold underline (3 cells): A2:C2
bg#D9EAD3 bold (2 cells): K1:K2
bg#F9CB9C (1 cells): E3
hidden rows: 4-308, 310-673, 675-1404

## 16. Mango · A1:N2000 · 21k cells · frozen 2r0c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Inventory Summary, Monthly Inventory Summary
same layout as "Avocado" (84% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Mango"`
blocks: A1:N2000 (20944 cells)
ƒ 14k cells in 10 groups; 1 families not in "Avocado":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=8961612","1/2!k3:p2000") [external] [array/spills]
labels not in "Avocado": D:"Mango (กรัม)" D:"มะม่วง (กรัม/1000บาท)"

## 17. MahachanokMango · A1:N2000 · 19k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage), Inventory Summary, Monthly Inventory Summary
same layout as "Avocado" (74% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "MahachanokMango"`
blocks: A1:N2000 (19053 cells)
ƒ 14k cells in 12 groups; 2 families not in "Avocado":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=8961612","1/2!s3:x2000") [external] [array/spills]
ƒ10 N3:N4 ↓ =if(and(M3<13%,M3>-5%),"Correct","Re-check") [2 errors]
labels not in "Avocado": D:"Mahachanok Mango (กรัม)" D:"มะม่วง (กรัม/1000บาท)" M:"% diff"

## 18. Pomelo · A1:V2000 · 32k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage), Inventory Summary, Monthly Inventory Summary
same layout as "Chicken" (69% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Pomelo"`
blocks: A1:V2000 (31893 cells)
ƒ 20k cells in 16 groups; 5 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","6/2!c3:h2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","3/1!n3:n2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","3/1!s3:s2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","3/1!p3:q2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","3/1!l3:l2000") [external] [array/spills]
labels not in "Chicken": A:"รายการ" D:"Pomelo (กรัม)" D:"ส้มโอ (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-เตรียมได้" Q:"% Yield"

## 19. Edamame · A1:V2000 · 31k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage), Inventory Summary, Monthly Inventory Summary
same layout as "Chicken" (63% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Edamame"`
blocks: A1:V2000 (31110 cells)
ƒ 20k cells in 16 groups; 6 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","3/1!s3:x2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/2!w3:w2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/2!ab3:ab2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/2!y3:z2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/2!u3:u2000") [external] [array/spills]
ƒ16 V3:V2000 ↓ =if(and(U3<=20%,U3>-5%),"Correct","Re-check") [304 errors]
labels not in "Chicken": A:"รายการ" D:"Edamame (กรัม)" D:"ถั่วแระ (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-เตรียมได้" Q:"% Yield" V:"-5%-20%"

## 20. Tamago · A1:V2005 · 32k cells · frozen 2r3c
reads: Sales (1g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage), Inventory Summary, Monthly Inventory Summary
same layout as "Chicken" (64% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Tamago"`
blocks: A1:V2005 (31582 cells)
ƒ 20k cells in 16 groups; 6 families not in "Chicken":
ƒ4 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","3/2!k3:p2000") [external] [array/spills]
ƒ5 K3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","2/1!n3:n2000") [external] [array/spills]
ƒ6 L3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","2/1!s3:s2000") [external] [array/spills]
ƒ7 M3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","2/1!p3:q2000") [external] [array/spills]
ƒ8 O3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","2/1!l3:l2000") [external] [array/spills]
ƒ12 R3:R4 ↓ =if(and(Q3<110%,Q3>95%),"Correct","Re-check")
labels not in "Chicken": A:"รายการ" D:"Tamago (กรัม)" D:"ไข่หวาน (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-เตรียมได้" Q:"% Yield" Q:"95-100%" V:"-5%-20%"

## 21. Eel · A1:R2000 · 26k cells · frozen 2r3c
reads: Sales (2g)
read by: Inventory Summary, Monthly Inventory Summary
▸ A1:R2000 table 2000×18 (25789 cells), header row 2
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Thu, Aug 19, 2027 (1998 filled)
  B "Month": ƒ1; num 1 … 12 (1998 filled)
  C "Year": ƒ2; num 2022 … 2027 (1998 filled)
  D "[ปลาไหล ] ปลาไหล (กรัม/1000บาท)": ƒ3,ƒ4; num 0.00 … 76.54 fmt 0.00; 304 errors (1998 filled)
  E "[2/2] คงเหลือ": num 600 … 6600 (1698 filled)
  F "ของเข้า(+)": num 2000 … 4600 (60 filled)
  G "ของออก(-)": num 200 … 1600 (4 filled)
  H "ไม่ได้คุณภาพ(-)": num 15 … 200 (3 filled)
  I "หมดอายุ(-)": empty
  J "ใช้ไป (ใบของสด)": num 0 … 2800 (1998 filled)
  K "[1/1] เตรียมได้": num 164 … 453 (1109 filled)
  L "ใช้ไป(ใบเตรียมของ) 5g": num 0 … 467 (1998 filled)
  M "เปิดร้าน (ใบเตรียมของ)": num 0 … 376 (1698 filled)
  N "(ใบนับของสด-เตรียมได้": ƒ9; num (210) … 2,800  fmt #,##0_);[Red](#,##0) (1998 filled)
  O "[100%] % Yield": ƒ11; num 0.00% … 145.00% fmt 0.00% (1998 filled)
  P "Check ใบของสด": ƒ12; enum Correct(1490) Re-check(508) (1998 filled)
  Q "[5/2] เตรียมได้(ใบเตรียมของ) 2.5g": num 21 … 401 (1213 filled)
  R "⏎ปลาไหลใช้ไป (5g)            …": ƒ10; num -213 … 360 (1998 filled)
  sample (A…P of 18):
  3| Tue, Mar 1, 2022 | 3 | 2022 | 13.45 | 1600 |  |  |  |  | 200 | 200 | 165 | 165 | 0  | 100.00% | Correct
  4| Wed, Mar 2, 2022 | 3 | 2022 | 13.33 | 1400 |  |  |  |  | 200 | 198 | 200 | 200 | 2  | 99.00% | Correct
  5| Thu, Mar 3, 2022 | 3 | 2022 | 0.00 | 1200 |  |  |  |  | 0 |  | 198 | 198 | 0  | 0.00% | Correct
  2000| Thu, Aug 19, 2027 | 8 | 2027 | #DIV/0! Function DIVIDE… |  |  |  |  |  | 0 |  | 0 |  | 0  | 0.00% | Correct
ƒ formulas (14k cells, 13 groups, 11 families)
ƒ1 B3:B2000 ↓ =month(A3)
ƒ2 C3:C2000 ↓ =year(A3)
ƒ3 D3:D1912 ↓ =J3/Sales!$G4*1000 [216 errors]
ƒ4 D1913:D2000 ↓ =J1913/Sales!$G2501*1000 [88 errors]
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=10481116","2/2!c3:h2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","1/1!e3:e2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","1/1!j3:j2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","1/1!c3:c2000") [external] [array/spills]
ƒ9 N3:N2000 ↓ =J3-K3
ƒ10 R3:R2000 ↓ =L3-Q3
ƒ11 O3:O2000 ↓ =iferror(K3/J3,0)
ƒ12 P3:P2000 ↓ =if(Or(and(O3<105%,O3>95%),and(N3=0,O3=0%)),"Correct","Re-check")
ƒ13 Q3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","5/2!n3:n2000") [external] [array/spills]
◐ conditional formats
P1:P2000, T3:T4: TEXT_EQ Correct → bg#B7E1CD
P1:P2000, T3:T4: TEXT_EQ Re-check → bg#EA9999
A1:A2000: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (21973 cells): D3:D2000, E4:E2000, F3:J2000, K4:M2000 +1
bg#EFEFEF (13986 cells): A3:C2000, N3:P2000, R3:R2000
bg#FCE5CD bold (22 cells): D1:M2, Q1:Q2
bg#EFEFEF bold (11 cells): A1:C1, N1:P2, R1:R2
bg#F9CB9C (5 cells): E3, K3:M3, Q3
bg#EFEFEF bold underline (3 cells): A2:C2
hidden rows: 4-307, 309-672, 675-1404

## 22. Ikura · A1:V2000 · 30k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage), Inventory Summary, Monthly Inventory Summary
same layout as "Chicken" (63% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Ikura"`
blocks: A1:V2000 (30111 cells)
ƒ 20k cells in 16 groups; 6 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=10481116","3/1!k3:p2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","1/2!n3:n2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","1/2!s3:s2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","1/2!p3:q2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=1940819478","1/2!l3:l2000") [external] [array/spills]
ƒ13 R3:R2000 ↓ =if(Or(and(Q3<=100%,Q3>95%),and(P3=0,Q3=0%)),"Correct","Re-check")
labels not in "Chicken": A:"รายการ" D:"Ikura (กรัม)" D:"ไข่ปลา (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-เตรียมได้" Q:"% Yield" U:"-5%-20%"

## 23. Others · A1:HW2011 · 271k cells · frozen 2r3c
reads: Sales (38g)
read by: Monthly Summary (Usage), Inventory Summary, Monthly Inventory Summary
▸ A1:HW2011 table 2011×231 (270529 cells), header row 2
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Mon, Aug 30, 2027 (2009 filled)
  B "Month": ƒ1; num 1 … 12 (2009 filled)
  C "Year": ƒ2; num 2022 … 2027 (2009 filled)
  D "[Engawa] ครีบปลา (กรัม/1000บาท)": ƒ3,ƒ22; num 0.00 … 11.61 fmt 0.00; 315 errors (2009 filled)
  E "[2/2] คงเหลือ": num 60 … 1200 (1698 filled)
  F "ของเข้า(+)": num 500 … 1000 (111 filled)
  G "ของออก(-)": G523=100
  H "ไม่ได้คุณภาพ(-) ": num 10 … 220 (93 filled)
  I "หมดอายุ(-)": I640=60
  J "ใช้ไป (ใบของสด)": num 0 … 640 (1998 filled)
  K "เปิดร้าน (ใบเตรียมของ)": num 0 … 160 (1698 filled)
  L "[1/1] ใช้ไป(ใบเตรียมของ) ": num 0 … 255 (1998 filled)
  M "เตรียมได้": num 30 … 300 (543 filled)
  N "[100%] % Yield": ƒ57; num 0% … 200% fmt 0% (2008 filled)
  O "[Engawa] Check ใบของสด": ƒ76; enum Correct(1942) Re-check(66) (2008 filled)
  P "[ก้ามปู] ก้ามปู (กรัม/1000บาท)": ƒ4,ƒ23; num -1.06 … 8.54 fmt 0.00; 314 errors (2008 filled)
  Q "[2/2] คงเหลือ": num 100 … 1300 (1698 filled)
  R "ของเข้า(+)": num 500 … 1000 (94 filled)
  S "ของออก(-)": S523=100
  T "ไม่ได้คุณภาพ(-)": num 10 … 100 (6 filled)
  U "หมดอายุ(-)": empty
  V "ใช้ไป (ใบของสด)": num 0 … 1000 (1998 filled)
  W "เปิดร้าน (ใบเตรียมของ)": num 0 … 200 (1698 filled)
  X "[1/1] ใช้ไป(ใบเตรียมของ) ": num -20 … 180 (1998 filled)
  Y "เตรียมได้": num 90 … 200 (473 filled)
  Z "[100%] % Yield": ƒ58; num 0% … 100% fmt 0% (2008 filled)
  AA "[ก้ามปู] Check ใบของสด": ƒ77; enum Correct(2000) Re-check(8) (2008 filled)
  AB "[กุ้ง] กุ้ง (กรัม/1000บาท)": ƒ5,ƒ24; num 0.00 … 1.22 fmt 0.00; 314 errors (2008 filled)
  AC "[3/1] คงเหลือ": num 0 … 130 (1698 filled)
  AD "ของเข้า(+)": num 30 … 120 (239 filled)
  AE "ของออก(-)": AE523=10
  AF "ไม่ได้คุณภาพ(-)": num 1 … 10 (5 filled)
  AG "หมดอายุ(-)": AG1610=1
  AH "ใช้ไป (ใบของสด)": num 0 … 90 (1998 filled)
  AI "เปิดร้าน (ใบเตรียมของ)": num 0 … 19 (1698 filled)
  AJ "[1/2] ใช้ไป(ใบเตรียมของ) ": num 0 … 23 (1998 filled)
  AK "เตรียมได้": num 8 … 30 (829 filled)
  AL "[100%] % Yield": ƒ59; num 0% … 100% fmt 0% (2009 filled)
  AM "[กุ้ง] Check ใบของสด": ƒ78; enum Correct(2002) Re-check(7) (2009 filled)
  AN "[ยำสาหร่าย] ยำสาหร่าย (กรัม/1000บาท)": ƒ6,ƒ25; num 0.00 … 35.55 fmt 0.00; 304 errors (1998 filled)
  +191 more columns
  sample (A…P of 231):
  3| Tue, Mar 1, 2022 | 3 | 2022 | 2.15 | 400 |  |  |  |  | 0 | 60 | 32 |  | 0% | Correct | 0.00
  4| Wed, Mar 2, 2022 | 3 | 2022 | 0.00 | 400 |  |  |  |  | 100 | 28 | 0 | 100 | 100% | Correct | 0.00
  5| Thu, Mar 3, 2022 | 3 | 2022 | 0.00 | 300 | 500 |  |  |  | 0 | 128 | 0 |  | 0% | Correct | 0.00
  2011| Mon, Aug 30, 2027 | 8 | 2027 | #DIV/0! Function DIVIDE… |  |  |  |  |  |  |  |  |  |  |  | 
ƒ formulas (118k cells, 157 groups, 25 families)
ƒ1 B3:B2011 ↓ =month(A3)
ƒ2 C3:C2011 ↓ =year(A3)
ƒ3–40 38 similar groups (37993 cells; same shape, refs/constants differ) e.g. D3:D1912 ↓ =L3/Sales!$G4*1000 [216 errors]; others at P3:P1912, AB3:AB1912, AN3:AN1912, AZ3:AZ1912, BL3:BL1912, BX3:BX1912 +31
ƒ41 E3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","2/2!K3:P2000") [external] [array/spills]
ƒ42–46 5 similar groups (5 cells; same shape, refs/constants differ) e.g. K3  =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/1!l3:l2000") [external] [array/spills]; others at CE3, EA3, FK3, GI3
ƒ47–51 5 similar groups (5 cells; same shape, refs/constants differ) e.g. L3  =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/1!s3:s2000") [external] [array/spills]; others at CF3, EB3, FL3, GJ3
ƒ52–56 5 similar groups (5 cells; same shape, refs/constants differ) e.g. M3  =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/1!n3:n2000") [external] [array/spills]; others at CG3, EC3, FM3, GK3
ƒ57–75 N3:N2010 ↓ =IFERROR(M3/(J3+H3),0) — same formula in 19 ranges (37993 cells): Z3:Z2010, AL3:AL2011, AX3:AX2000, BJ3:BJ2000, BV3:BV2000, CH3:CH2000 +12; breaks at AY3:BI2000, BK3:BU2000, BW3:CG2000, CI3:CS2000, CU3:DE2000 +11 = 125590 hardcoded values, 64023 different formulas, 162145 blank
ƒ76–93 18 similar groups (35995 cells; same shape, refs/constants differ) e.g. O3:O2010 ↓ =if(Or(and(N3<=100%,N3>90%),and(M3=0,N3=0%)),"Correct","Re-check"); others at AA3:AA2010, AM3:AM2011, AY3:AY2000, BK3:BK2000, BW3:BW2000, CI3:CI2000 +11
ƒ94–100 7 similar groups (7 cells; same shape, refs/constants differ) e.g. Q3  =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","2/2!s3:x2000") [external] [array/spills]; others at BY3, CW3, EG3, FQ3, GO3, HA3
ƒ101–108 8 similar groups (8 cells; same shape, refs/constants differ) e.g. W3  =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/1!u3:u2000") [external] [array/spills]; others at BG3, CQ3, DC3, EM3, FW3, GU3 +1
ƒ109–116 8 similar groups (8 cells; same shape, refs/constants differ) e.g. X3  =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/1!ab3:ab2000") [external] [array/spills]; others at BH3, CR3, DD3, EN3, FX3, GV3 +1
ƒ117–124 8 similar groups (8 cells; same shape, refs/constants differ) e.g. Y3  =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/1!w3:w2000") [external] [array/spills]; others at BI3, CS3, DE3, EO3, FY3, GW3 +1
ƒ125–128 4 similar groups (4 cells; same shape, refs/constants differ) e.g. AC3  =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","3/1!c3:h2000") [external] [array/spills]; others at AO3, BM3, CK3
ƒ129–133 5 similar groups (5 cells; same shape, refs/constants differ) e.g. AI3  =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/2!c3:c2000") [external] [array/spills]; others at AU3, BS3, DO3, EY3
ƒ134–138 5 similar groups (5 cells; same shape, refs/constants differ) e.g. AJ3  =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/2!j3:j2000") [external] [array/spills]; others at AV3, BT3, DP3, EZ3
ƒ139–143 5 similar groups (5 cells; same shape, refs/constants differ) e.g. AK3  =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","1/2!e3:e2000") [external] [array/spills]; others at AW3, BU3, DQ3, FA3
ƒ144 BA3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","3/2!s3:v2000") [external] [array/spills]
ƒ145 BE3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","2/1!z3:z2000") [external] [array/spills]
ƒ146 BF3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","3/2!x3:x2000") [external] [array/spills]
ƒ147 DI3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","7/2!c3:f2000") [external] [array/spills]
ƒ148 ES3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","8/1!c3:f2000") [external] [array/spills]
ƒ149 DM3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","3/2!h3:h2000") [external] [array/spills]
ƒ150 EW3 =importrange("https://docs.google.com/spreadsheets/d/1VWvEM2GtYtS-qcS5yc3dpLN34yCp993ZiOkgJX837B0/edit#gid=381150158","4/1!h3:h2000") [external] [array/spills]
ƒ151 DN3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","7/2!h3:h2000") [external] [array/spills]
ƒ152 EX3 =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","8/1!h3:h2000") [external] [array/spills]
ƒ153–156 4 similar groups (4 cells; same shape, refs/constants differ) e.g. DU3  =importrange("https://docs.google.com/spreadsheets/d/1-7utEJCqAYaJ0qTT2sfsB047uzjGDSvbRX9joROd00c/edit#gid=1435095814","7/2!k3:p2000") [external] [array/spills]; others at FE3, GC3, HM3
ƒ157 GM3:GM2000 ↓ =if(Or(and(GL3<=100%,GL3>=40%),and(GK3=0,GL3=0%)),"Correct","Re-check")
◐ conditional formats
O1:O3000, AA1:AA3000, AB1 +56: TEXT_EQ Correct → bg#B7E1CD
O1:O3000, AA1:AA3000, AB1 +56: TEXT_EQ Re-check → bg#EA9999
A1:A3000: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (345497 cells): D3:D2011, E4:E2011, F3:J2011, K4:M2011 +74
bg#EFEFEF (118513 cells): A3:C2011, N3:O2011, Y4:Y2011, Z3:AA2011 +34
bg#FCE5CD bold (367 cells): D1:M2, P1:Y2, AB1:AK2, AN1:AW2 +19
bg#F9CB9C (82 cells): E3, K3:M3, Q3, W3:Y3 +34
bg#EFEFEF bold (79 cells): A1:C1, N1:O2, Z1:AA2, AL1:AM2 +16
bg#EFEFEF bold underline (3 cells): A2:C2
hidden rows: 4-308, 310-673, 675-703, 705-732, 734-763, 765-1404
hidden cols: DI-DQ, HA-HI, HM-HU
