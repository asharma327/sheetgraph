# ใบตรวจการใช้วัตถุดิบ (TCM)
gsheet 1eAE0enn5IpCKv0_rLAPuzI0kABWwsk9-nktymph0KbE · en_US · Asia/Bangkok · modified 2026-10-23 · owner accounting@hunterpoke.com
23 sheets · 2.7M non-empty cells · 1.4M formulas in 3254 copy-groups · 59k error cells
Notation: ƒN = formula copy-group N of a sheet; a group's formula is shown at its top-left cell and is copied (relative refs shift) over its range. ↓ filled down, → across, ▦ both.

## Sheets
1. Template — A1:C3 · 4 cells
2. Sales — A1:L2000 · 24k cells · 1 ƒ in 1 groups
3. Monthly Summary (item sales) — B1:HD1124 · 22k cells · 18k ƒ in 728 groups · reads Item Sales (POS)
4. Monthly Summary (Usage) — C1:BZ249 · 11k cells · 11k ƒ in 244 groups · reads Sales, Item Sales (Cal), Item Sales (POS), Fish record, Salmon, Tuna, Chicken, Chicken BB, Mango, Avocado, MahachanokMango, Pomelo, Tamago, Edamame, Ikura, Others
5. Item Sales (Cal) — A1:BOM1145 · 969k cells · 968k ƒ in 1387 groups · reads Item Sales (POS)
6. Item Sales (POS) — A1:BPC916 · 997k cells · 11k ƒ in 18 groups
7. Item Sales (Usage) — A2:M2000 · 23k cells · 21k ƒ in 12 groups · reads Item Sales (Cal)
8. Monthly Inventory Summary — A1:AE35 · 941 cells · 835 ƒ in 429 groups · reads Salmon, Tuna, Mango, Avocado, MahachanokMango, Pomelo, Tamago, Edamame, Eel, Ikura, Others
9. Inventory Summary — A1:I34 · 117 cells · 78 ƒ in 78 groups · reads Fish record, Salmon, Tuna, Mango, Avocado, MahachanokMango, Pomelo, Tamago, Edamame, Eel, Ikura, Others
10. Fish record — A1:W1927 · 27k cells · 12k ƒ in 9 groups
11. Salmon — A1:V2001 · 36k cells · 28k ƒ in 23 groups · reads Sales, Item Sales (Usage), Fish record
12. Tuna — A1:V2000 · 36k cells · 28k ƒ in 17 groups · reads Sales, Item Sales (Usage), Fish record
13. Chicken — A1:V1929 · 28k cells · 19k ƒ in 16 groups · reads Sales, Item Sales (Usage)
14. Chicken BB — A1:V1929 · 25k cells · 19k ƒ in 17 groups · reads Sales, Item Sales (Usage)
15. Mango — A1:N2000 · 21k cells · 14k ƒ in 10 groups · reads Sales, Item Sales (Usage)
16. Avocado — A1:N2000 · 22k cells · 14k ƒ in 9 groups · reads Sales, Item Sales (Usage)
17. MahachanokMango — A1:N2000 · 19k cells · 14k ƒ in 13 groups · reads Sales, Item Sales (Usage)
18. Pomelo — A1:V2000 · 32k cells · 20k ƒ in 16 groups · reads Sales, Item Sales (Usage)
19. Tamago — A1:V2000 · 32k cells · 20k ƒ in 19 groups · reads Sales, Item Sales (Usage)
20. Edamame — A1:V2000 · 31k cells · 20k ƒ in 17 groups · reads Sales, Item Sales (Usage)
21. Eel — A1:R2000 · 26k cells · 14k ƒ in 13 groups · reads Sales
22. Ikura — A1:V2000 · 30k cells · 20k ƒ in 17 groups · reads Sales, Item Sales (Usage)
23. Others — A1:HW3000 · 324k cells · 171k ƒ in 161 groups · reads Sales

## Data flow (source → readers, by formula refs)
Sales → Monthly Summary (Usage) (6g), Salmon (2g), Tuna (2g), Chicken (1g), Chicken BB (1g), Mango (2g), Avocado (2g), MahachanokMango (2g), Pomelo (2g), Tamago (2g), Edamame (2g), Eel (2g), Ikura (2g), Others (40g)
Item Sales (Cal) → Monthly Summary (Usage) (8g), Item Sales (Usage) (10g)
Item Sales (POS) → Monthly Summary (item sales) (185g), Monthly Summary (Usage) (1g), Item Sales (Cal) (1308g)
Item Sales (Usage) → Salmon (1g), Tuna (1g), Chicken (1g), Chicken BB (1g), Mango (2g), Avocado (1g), MahachanokMango (1g), Pomelo (1g), Tamago (2g), Edamame (2g), Ikura (2g)
Fish record → Monthly Summary (Usage) (16g), Inventory Summary (6g), Salmon (5g), Tuna (3g)
Salmon → Monthly Summary (Usage) (4g), Monthly Inventory Summary (14g), Inventory Summary (3g)
Tuna → Monthly Summary (Usage) (4g), Monthly Inventory Summary (14g), Inventory Summary (2g)
Chicken → Monthly Summary (Usage) (14g)
Chicken BB → Monthly Summary (Usage) (1g)
Mango → Monthly Summary (Usage) (8g), Monthly Inventory Summary (7g), Inventory Summary (3g)
Avocado → Monthly Summary (Usage) (8g), Monthly Inventory Summary (7g), Inventory Summary (3g)
MahachanokMango → Monthly Summary (Usage) (8g), Monthly Inventory Summary (7g), Inventory Summary (1g)
Pomelo → Monthly Summary (Usage) (14g), Monthly Inventory Summary (14g), Inventory Summary (2g)
Tamago → Monthly Summary (Usage) (14g), Monthly Inventory Summary (14g), Inventory Summary (5g)
Edamame → Monthly Summary (Usage) (14g), Monthly Inventory Summary (14g), Inventory Summary (5g)
Eel → Monthly Inventory Summary (14g), Inventory Summary (2g)
Ikura → Monthly Summary (Usage) (14g), Monthly Inventory Summary (14g), Inventory Summary (2g)
Others → Monthly Summary (Usage) (36g), Monthly Inventory Summary (259g), Inventory Summary (42g)

## External links
IMPORTRANGE 19pXjj0ZWWhr3kcWDRrT3w3Dxd88elcT1x1rKM1I1W6M (1): "Sales(ใบตรวจร้าน)!A1:L2000"→Sales!A1
IMPORTRANGE 1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ (39): "Fish record summary!D3:g2000"→'Fish record'!D3, "Fish record summary!j3:m2000"→'Fish record'!J3, "1/1!c3:h2000"→Salmon!E4, "1/1!k3:k1000"→Salmon!V4, "1/2!c3:h11"→Tuna!E3, "1/2!c3:c1000"→Tuna!V3, "1/1!s12:x2000"→Tuna!E12, "9/2!K460:P2000"→Chicken!E3, "9/2!C460:H2000"→'Chicken BB'!E3, "1/2!k3:p2000"→Mango!E3, "2/1!c3:h2000"→Avocado!E3, "1/2!s3:x2000"→MahachanokMango!E3 +27
IMPORTRANGE 1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs (87): "5/3!E460:E2000"→Chicken!K3, "5/3!J460:J2000"→Chicken!L3, "5/3!G460:G2000"→Chicken!M3, "5/3!H460:H2000"→Chicken!N3, "5/3!C460:C2000"→Chicken!O3, "5/3!N460:N2000"→'Chicken BB'!K3, "5/3!S460:S2000"→'Chicken BB'!L3, "5/3!P460:P2000"→'Chicken BB'!M3, "5/3!Q460:Q2000"→'Chicken BB'!N3, "5/3!L460:L2000"→'Chicken BB'!O3, "3/1!n3:n2000"→Pomelo!K3, "3/1!s3:s2000"→Pomelo!L3 +75

## Functions (cells using)
AND 162k · IF 94k · IFERROR 71k · OR 68k · YEAR 34k · MONTH 34k · SUMIFS 32k · HLOOKUP 17k · SUM 13k · DAY 1818 · SUMPRODUCT 1425 · INDEX 454 · MATCH 454 · IMPORTRANGE 127 · EOMONTH 57 · TODAY 2
volatile: TODAY 2

## Extraction notes
- Apps Script (bound scripts, macros, add-ons, triggers) is not exposed by the Sheets API; pass --apps-script <scriptId> with user credentials to include a project

## 1. Template · A1:C3 · 4 cells
▸ A1:C3 (4 cells)
  1: A=  C=importrange
  2: C=Manual cal
  3: C=Formula
◆ formatting
bg#FCE5CD (1 cells): B1
bg#D9EAD3 (1 cells): B2
bg#EFEFEF (1 cells): B3

## 2. Sales · A1:L2000 · 24k cells
read by: Monthly Summary (Usage), Salmon, Tuna, Chicken, Chicken BB, Mango, Avocado, MahachanokMango, Pomelo, Tamago, Edamame, Eel, Ikura, Others
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

## 3. Monthly Summary (item sales) · B1:HD1124 · 22k cells · frozen 0r3c
reads: Item Sales (POS) (185g)
▸ BQ4:GO32 table 29×129 (2658 cells), header row 4
  BQ -: ƒ36,ƒ291,ƒ352; num 0.00% … 659 fmt 0.00% (16 filled)
  BR -: ƒ37; num 0.00 … 1,588.50 fmt #,##0.00; text "Total 6/2025" (28 filled)
  BS -: ƒ38,ƒ292,ƒ353; num 0.00% … 1,780 fmt 0.00% (16 filled)
  BT -: num 0.00 … 438.50 fmt #,##0.00; text "CSD 6/2025" (28 filled)
  BU -: ƒ39,ƒ293,ƒ354; num 0.00% … 496 fmt 0.00% (16 filled)
  BV -: num 0.00 … 557.00 fmt #,##0.00; text "EMQ 6/2025" (28 filled)
  BW -: ƒ40,ƒ294,ƒ355; num 0.00% … 625 fmt 0.00% (16 filled)
  BX -: ƒ41,ƒ295,ƒ356; num 0.00% … 550 fmt 0.00% (16 filled)
  BY -: ƒ42; num 0.00 … 1,499.00 fmt #,##0.00; text "Total 5/2025" (28 filled)
  BZ -: ƒ43,ƒ296,ƒ357; num 0.00% … 1,652 fmt 0.00% (16 filled)
  CA -: num 0.00 … 427.00 fmt #,##0.00; text "CSD 5/2025" (28 filled)
  CB -: ƒ44,ƒ297,ƒ358; num 0.00% … 474 fmt 0.00% (16 filled)
  CC -: num 0.00 … 568.50 fmt #,##0.00; text "EMQ 5/2025" (28 filled)
  CD -: ƒ45,ƒ298,ƒ359; num 0.00% … 629 fmt 0.00% (16 filled)
  CE -: ƒ46,ƒ299,ƒ360; num 0.00% … 573 fmt 0.00% (16 filled)
  CF -: ƒ47; num 0.00 … 1,469.00 fmt 0.00%; text "Total 4/2025" (28 filled)
  CG -: ƒ48,ƒ300,ƒ361; num 0.00% … 1,656 fmt 0.00% (16 filled)
  CH -: num 0.00 … 435.50 fmt #,##0.00; text "CSD 4/2025" (28 filled)
  CI -: ƒ49,ƒ301,ƒ362; num 0.00% … 513 fmt 0.00% (16 filled)
  CJ -: num 0.00 … 519.50 fmt #,##0.00; text "EMQ 4/2025" (28 filled)
  CK -: ƒ50,ƒ302,ƒ363; num 0.00% … 571 fmt 0.00% (16 filled)
  CM -: ƒ51,ƒ303; num 0.00% … 653 fmt 0.00% (27 filled)
  CN -: ƒ52; num 0 … 1605.5; text "Total 3/2025" (28 filled)
  CO -: ƒ53,ƒ304; num 0.00% … 1,779 fmt 0.00% (27 filled)
  CP -: num 0 … 473.5; text "CSD 3/2025" (28 filled)
  CQ -: ƒ54,ƒ305; num 0.00% … 533 fmt 0.00% (27 filled)
  CR -: num 0 … 542; text "EMQ 3/2025" (28 filled)
  CS -: ƒ55,ƒ306; num 0.00% … 593 fmt 0.00% (27 filled)
  CT -: ƒ56,ƒ307; num 0.00% … 639 fmt 0.00% (27 filled)
  CU -: ƒ57; num 0 … 1480.5; text "Total" (27 filled)
  CV -: ƒ58,ƒ308; num 0.00% … 1,656 fmt 0.00% (26 filled)
  CW -: num 0 … 398; text "CSD 2/2025" (28 filled)
  CX -: ƒ59,ƒ309; num 0.00% … 479 fmt 0.00% (27 filled)
  CY -: num 0 … 491.5; text "EMQ 2/2025" (28 filled)
  CZ -: ƒ60,ƒ310; num 0.00% … 538 fmt 0.00% (27 filled)
  DA -: ƒ61,ƒ311,ƒ364; num 0.00% … 840 fmt 0.00% (16 filled)
  DB "Total": ƒ62; num 0.0 … 1784.0 fmt 0.0 (27 filled)
  DC -: ƒ63,ƒ312,ƒ365; num 0.00% … 2,014 fmt 0.00% (16 filled)
  DD "CSD 1/2025": num 0 … 449 (27 filled)
  DE -: ƒ64,ƒ313,ƒ366; num 0.00% … 543 fmt 0.00% (16 filled)
  +75 more columns
  sample (BQ…CF of 129):
  5|  | Total 6/2025 |  | CSD 6/2025 |  | EMQ 6/2025 |  |  | Total 5/2025 |  | CSD 5/2025 |  | EMQ 5/2025 |  |  | Total 4/2025
  6| 659 | 1,588.50 | 1,780 | 438.50 | 496 | 557.00 | 625 | 550 | 1,499.00 | 1,652 | 427.00 | 474 | 568.50 | 629 | 573 | 1,469.00
  7| 17.30% | 304.50 | 17.11% | 85.00 | 17.15% | 105.50 | 16.88% | 14.29% | 267.50 | 16.19% | 96.00 | 20.25% | 93.00 | 14.80% | 16.40% | 282.50
  32|  | 2.00 |  | 1.00 |  | 0.00 |  |  | 5.00 |  | 3.00 |  | 2.00 |  |  | 3.00
▸ C5:BM32 row-oriented 28×63 (1477 cells), labels in column C
  5 -: ƒ7,ƒ8; num -100.00% … 17.78% fmt 0.00%; 4 errors (41 filled)
  6…32 (27 alike) "In-store" "The Classic" "BYO" "X Factor" "California" "Sexy Salmon" "K bowl" "Triple sea" "Pimpim pesto" "Chipotle" "Hunter's bowl" "Tofu emperor" "Avo-mango balsamic" "Triple sea (no engawa)" "Chicken bowl" "instore Chicken Rosa" "instor…: ƒ195,ƒ196; num 0 … 1102.5 fmt 0.0 (61 filled)
▸ C34:BM60 row-oriented 27×63 (1407 cells), labels in column C
  34 -: ƒ18,ƒ19; num -100.00% … 24.38% fmt 0.00%; 4 errors (38 filled)
  35…60 (26 alike) "Delivery" "The Classic" "BYO" "X Factor" "California" "Sexy Salmon" "K bowl" "Triple sea" "Pimpim pesto" "Chipotle" "Hunter's bowl" "Tofu emperor" "Avo-mango balsamic" "Chicken bowl" "Deli Chicken Rosa" "Deli Chicken Black Truffle" "Deli …: ƒ210,ƒ211,ƒ212; num 0 … 1650.5 fmt 0.0 (61 filled)
▸ BQ34:DG60 table 27×43 (923 cells), no header row
  BQ -: ƒ109,ƒ400,ƒ459; num 0.00% … 1388 fmt 0.00% (15 filled)
  BR -: ƒ110; num 0.00 … 2,374.00 fmt #,##0.00 (26 filled)
  BS -: ƒ111,ƒ401,ƒ460; num 0.00% … 3,198.00 fmt 0.00% (15 filled)
  BT -: num -1.94% … 708.00 fmt #,##0.00 (27 filled)
  BU -: ƒ112,ƒ402,ƒ461; num 0.00% … 919.00 fmt 0.00% (15 filled)
  BV -: num 0.00 … 743.00 fmt #,##0.00 (27 filled)
  BW -: ƒ113,ƒ403,ƒ462; num 0.00% … 891.00 fmt 0.00% (15 filled)
  BX -: ƒ114,ƒ404,ƒ463; num 0.00% … 1446.5 fmt 0.00% (15 filled)
  BY -: ƒ115; num 0.00 … 2,342.00 fmt #,##0.00 (26 filled)
  BZ -: ƒ116,ƒ405,ƒ464; num 0.00% … 3,189.00 fmt 0.00% (15 filled)
  CA -: num -6.17% … 722.00 fmt #,##0.00 (27 filled)
  CB -: ƒ117,ƒ406,ƒ465; num 0.00% … 971.00 fmt 0.00% (15 filled)
  CC -: num 0.00 … 629.50 fmt #,##0.00 (27 filled)
  CD -: ƒ118,ƒ407,ƒ466; num 0.00% … 771.50 fmt 0.00% (15 filled)
  CE -: ƒ119,ƒ408,ƒ467; num 0.00% … 1443.5 fmt 0.00% (15 filled)
  CF -: ƒ120; num 0.00 … 2,333.50 (26 filled)
  CG -: ƒ121,ƒ409,ƒ468; num 0.00% … 3,214.50 fmt 0.00% (15 filled)
  CH -: num 0.00 … 769.50 fmt #,##0.00 (26 filled)
  CI -: ƒ122,ƒ410,ƒ469; num 0.00% … 1,051.50 fmt 0.00% (15 filled)
  CJ -: num 0.00 … 584.50 fmt #,##0.00 (26 filled)
  CK -: ƒ123,ƒ411,ƒ470; num 0.00% … 719.50 fmt 0.00% (15 filled)
  CM -: ƒ124,ƒ412; num 0.00% … 1683.5 fmt 0.00% (26 filled)
  CN -: ƒ125; num 0 … 2786 (26 filled)
  CO -: ƒ126,ƒ413; num 0.00% … 3824 fmt 0.00% (26 filled)
  CP -: num 0 … 1001.5 fmt 0.00% (27 filled)
  CQ -: ƒ127,ƒ414; num 0.00% … 1398.5 fmt 0.00% (26 filled)
  CR -: num 0 … 601 (26 filled)
  CS -: ƒ128,ƒ415; num 0.00% … 742 fmt 0.00% (26 filled)
  CT -: ƒ129,ƒ416; num 0.00% … 1349.5 fmt 0.00% (26 filled)
  CU -: ƒ130; num 0 … 2206.5 (26 filled)
  CV -: ƒ131,ƒ417; num 0.00% … 3015.5 fmt 0.00% (26 filled)
  CW -: num 0 … 720 (26 filled)
  CX -: ƒ132,ƒ418; num 0.00% … 987 fmt 0.00% (26 filled)
  CY -: num 0 … 535 (26 filled)
  CZ -: ƒ133,ƒ419; num 0.00% … 679 fmt 0.00% (26 filled)
  DA -: ƒ134,ƒ420,ƒ471; num 0.00% … 1665 fmt 0.00% (15 filled)
  DB -: ƒ135; num 0.0 … 2541.5 fmt 0.0 (26 filled)
  DC -: ƒ136,ƒ421,ƒ472; num 0.00% … 3461.5 fmt 0.00% (15 filled)
  DD -: num 0 … 841.5 fmt 0.00% (27 filled)
  DE -: ƒ137,ƒ422,ƒ473; num 0.00% … 1159.5 fmt 0.00% (15 filled)
  +2 more columns
  sample (BQ…CF of 43):
  34|  |  |  | -1.94% |  | 18.03% |  |  |  |  | -6.17% |  | 7.70% |  |  | 
  35| 1388 | 2,374.00 | 3,198.00 | 708.00 | 919.00 | 743.00 | 891.00 | 1446.5 | 2,342.00 | 3,189.00 | 722.00 | 971.00 | 629.50 | 771.50 | 1443.5 | 2,333.50
  36| 17.15% | 591.00 | 18.48% | 164.00 | 17.85% | 189.00 | 21.21% | 23.16% | 698.00 | 21.89% | 227.00 | 23.38% | 136.00 | 17.63% | 21.30% | 635.00
  60|  | 12.00 |  | 3.00 |  | 1.00 |  |  | 13.00 |  | 6.00 |  | 3.00 |  |  | 11.00
▸ DJ34:GO60 table 27×84 (1531 cells), no header row
  DJ -: ƒ139,ƒ424,ƒ475; num 0.00% … 1514.5 fmt 0.00% (15 filled)
  DL -: ƒ140; num 0.0 … 2314.5 fmt 0.0 (26 filled)
  DM -: ƒ141,ƒ425; num 0.00% … 3304.5 fmt 0.00% (26 filled)
  DN -: num 0.0 … 824.0 fmt 0.0 (26 filled)
  DO -: ƒ142,ƒ426; num 0.00% … 1170.0 fmt 0.00% (26 filled)
  DP -: num 0.0 … 486.0 fmt 0.0 (26 filled)
  DQ -: ƒ143,ƒ427; num 0.00% … 620.0 fmt 0.00% (26 filled)
  DS -: ƒ144; num 0.0 … 2591.5 fmt 0.0 (26 filled)
  DT -: ƒ145,ƒ428,ƒ476; num 0.00% … 3540.5 fmt 0.00% (15 filled)
  DU -: num -23.72% … 940.0 fmt 0.0 (27 filled)
  DV -: ƒ146,ƒ429,ƒ477; num 0.00% … 1317.0 fmt 0.00% (15 filled)
  DW -: num -9.23% … 518.0 fmt 0.0 (27 filled)
  DX…EC (5 alike): ƒ147,ƒ430,ƒ478; num 0.00% … 651.0 fmt 0.00% (15 filled)
  ED -: num 0 … 1232 (26 filled)
  EE -: ƒ150,ƒ433,ƒ481; num 0.00% … 1641 fmt 0.00% (23 filled)
  EF -: num 0 … 569.5 (26 filled)
  EG…EJ (4 alike): ƒ151,ƒ434,ƒ482; num 0.00% … 704.5 fmt 0.00% (23 filled)
  EK -: num 0 … 1,102 fmt #,##0 (26 filled)
  EL -: ƒ154,ƒ437,ƒ485; num 0.00% … 1,488 fmt 0.00% (15 filled)
  EM -: num 0 … 512 fmt #,##0 (26 filled)
  EN…EQ (4 alike): ƒ155,ƒ438,ƒ486; num 0.00% … 618 fmt 0.00% (15 filled)
  ER -: num 0 … 1,193 fmt #,##0 (27 filled)
  ES -: num 0 … 526 fmt #,##0 (27 filled)
  EU -: ƒ158,ƒ441,ƒ489; num 0.00% … 1617 fmt 0.00% (15 filled)
  EV -: ƒ20,ƒ217,ƒ177; num -5.80% … 2,633 fmt #,##0 (27 filled)
  EW -: ƒ159,ƒ442,ƒ490; num 0.00% … 3,481 fmt 0.00% (15 filled)
  EX -: num 0 … 965 fmt #,##0 (27 filled)
  EY -: num 0 … 451 fmt #,##0 (27 filled)
  EZ -: ƒ160,ƒ443,ƒ491; num 0.00% … 1568.5 fmt 0.00% (15 filled)
  FB -: ƒ21,ƒ218,ƒ178 +1; num -9.53% … 2,795 fmt #,##0 (16 filled)
  FC -: ƒ161,ƒ444,ƒ492; num 0.00% … 3,467 fmt 0.00% (15 filled)
  FD -: num 0 … 1063 (26 filled)
  FE -: num 0 … 512 (26 filled)
  FF -: ƒ162,ƒ445,ƒ493; num 0.00% … 1715 fmt 0.00% (15 filled)
  FG -: ƒ22,ƒ219,ƒ179; num 0 … 3,089 fmt 0.00% (27 filled)
  FH -: ƒ163,ƒ446,ƒ494; num 0.00% … 3,697 fmt 0.00% (15 filled)
  FI -: num 0 … 1,183 fmt #,##0 (26 filled)
  FJ -: num 0 … 513 fmt 0 (26 filled)
  FL -: ƒ23,ƒ220,ƒ180; num -5.57% … 2679 fmt 0.00% (25 filled)
  FM -: ƒ164,ƒ447,ƒ495; num 0.00% … 3226 fmt 0.00% (15 filled)
  +22 more columns
  sample (DJ…DY of 84):
  34|  |  |  |  |  |  |  |  |  |  |  | -23.72% |  | -9.23% |  | 
  35| 1514.5 |  | 2314.5 | 3304.5 | 824.0 | 1170.0 | 486.0 | 620.0 |  | 2591.5 | 3540.5 | 940.0 | 1317.0 | 518.0 | 651.0 | 
  36| 18.45% |  | 694.5 | 21.02% | 287.5 | 24.57% | 127.5 | 20.56% |  | 721.0 | 20.36% | 276.0 | 20.96% | 137.5 | 21.12% | 
  60|  |  | 12 | 0.36% | 4 | 0.34% | 2 | 0.32% |  | 16 |  | 8 |  | 4 |  | 
▸ B62:AH101 table 40×33 (1249 cells), no header row
  B -: num 1 … 1.5 (39 filled)
  C -: text 40 distinct e.g. "Avo-Mango Balsamic Sala…", "Avo-Mango Balsamic Sala…", "Avo-Mango Balsamic Sala…" (40 filled)
  E…AH (30 alike): ƒ541,ƒ543,ƒ545 +6; num 0 … 205 (39 filled)
  sample (B…Q of 33):
  62|  | Instore |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  63| 1 | The Classic - สแตนดาร์ด |  | 205 | 184 | 160 | 146 | 159 | 207 | 192 | 150 | 122 | 200 | 195 | 138 | 185
  64| 1 | The Classic - พิเศษ |  | 69 | 77 | 51 | 38 | 30 | 59 | 48 | 64 | 32 | 59 | 82 | 31 | 43
  101| 1.5 | Mayongchid  |  | 6 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
▸ AM63:BM101 table 39×27 (1053 cells), no header row
  AM…BM (27 alike): ƒ542,ƒ544,ƒ546 +6; num 0 … 176 (39 filled)
  sample (AM…BB of 27):
  63| 88 | 93 | 83 | 73 | 59 | 95 | 83 | 90 | 69 | 61 | 61 | 74 | 54 | 69 | 73 | 55
  64| 18 | 16 | 21 | 15 | 18 | 16 | 20 | 25 | 10 | 16 | 12 | 17 | 22 | 19 | 21 | 12
  65| 2 | 3 | 4 | 4 | 1 | 2 | 4 | 2 | 0 | 3 | 4 | 5 | 1 | 0 | 1 | 2
  101| 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
▸ B104:AH141 table 38×33 (1154 cells), no header row
  B -: num 1 … 1.5 (35 filled)
  C -: text 36 distinct e.g. "Avo-mango Deli - พิเศษ", "Avo-mango Deli - สแตนดา…", "BYO Deli" (36 filled)
  E…AH (30 alike): ƒ559,ƒ561,ƒ563 +10; num 0 … 322 (35 filled)
  sample (B…Q of 33):
  104|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  105|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  106|  | Delivery |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  141| 1 | Mayongchid Deli |  | 24 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 4
▸ AM104:BM141 table 38×27 (1026 cells), no header row
  AM…BH (22 alike): ƒ508,ƒ723,ƒ228 +13; num 0 … 1158 fmt 0.00% (38 filled)
  BI…BM (5 alike): ƒ508,ƒ723,ƒ228 +13; num 0 fmt 0.00%; 1 errors (38 filled)
  sample (AM…BB of 27):
  104| 3.97% | 2.84% | 2.62% | 2.55% | 2.14% | 3.52% | 2.40% | 3.46% | 1.19% | 1.77% | 2.68% | 4.20% | 3.12% | 2.90% | 2.90% | 2.84%
  105| 46 | 27 | 31 | 25 | 21 | 32 | 19 | 30 | 10 | 16 | 24 | 37 | 32 | 26 | 30 | 30
  106| 1158 | 952 | 1185 | 980 | 983 | 909 | 793 | 868 | 838 | 902 | 897 | 880 | 1026 | 898 | 1033 | 1056
  141| 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
▸ C144:AH326 table 183×32 (2767 cells), no header row
  C -: text 183 distinct e.g. "(Free voucher) Chicken …", "(Free voucher) Chicken …", "Buy1 Get1 Free SET 1 : …" (183 filled)
  E…AH (30 alike): ƒ585,ƒ587,ƒ589 +2; num 0 … 45 (11 filled)
  sample (C…R of 32):
  144| Promotion |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
  145| Trip + KBowl (FPD 10.1-… |  | 18 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  146| Cali + Hun (FPD 10.1-31… |  | 42 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  326| x factor chicken |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
▸ AM145:BM249 table 105×27 (2835 cells), no header row
  AM…BM (27 alike): ƒ586,ƒ588,ƒ590 +46; num 0 … 513 (105 filled)
  sample (AM…BB of 27):
  145| 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  146| 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  147| 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  249| 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
▸ AM259:BM326 table 68×27 (1780 cells), no header row
  AM…BM (27 alike): ƒ688,ƒ689,ƒ691 +6; num 0 … 156 (61 filled)
  sample (AM…BB of 27):
  259| 35 | 15 | 15 | 20 | 18 | 19 | 18 | 14 | 10 | 3 | 3 | 8 | 14 | 13 | 24 | 15
  260| 5 | 0 | 2 | 1 | 1 | 0 | 2 | 4 | 1 | 2 | 3 | 6 | 6 | 2 | 2 | 4
  261| 13 | 4 | 10 | 4 | 1 | 1 | 6 | 5 | 7 | 6 | 3 | 2 | 4 | 1 | 0 | 3
  326|  |  |  |  |  |  |  |  | 1 | 41 | 42 | 50 | 66 | 16 | 16 | 19
▸ +35 smaller blocks (1687 cells): AM338:BM349, AM328:BM336, C1:BM3, GS35:HD49, GS6:HC20, Y338:AH349, AA352:AH371, Y328:AH336 +27
ƒ formulas (18k cells, 728 groups, 19 families)
ƒ1–2 E1:AJ1 → =year(E3) — same formula in 2 ranges (59 cells): AM1:BM1; breaks at AK1:AL1 = 2 hardcoded values
ƒ3–5 E2:AJ2 → =month(E3) — same formula in 3 ranges (56 cells): AM2:AU2, AY2:BM2; breaks at AK2:AL2, AV2:AX2 = 5 hardcoded values
ƒ6 BB3:BM3 → =eomonth(BA3,1)
ƒ7–26 20 similar groups (95 cells; same shape, refs/constants differ) e.g. W5:AJ5 → =(W6-V6)/V6; others at AM5:BM5, EA5, EI5, EP5, EV5, FB5 +13
ƒ27–33 7 similar groups (316 cells; same shape, refs/constants differ) e.g. FW5  =FX5+FY5+AC6+AC20+AC24; others at AM53:BM54, AM56:BM56, AM57:BM58, AM59:BM60, T240:AH242, AM240:BM242
ƒ34–194 161 similar groups (1352 cells; same shape, refs/constants differ) e.g. FX5:FY5 → =FX6+FX20+FX24; others at GT5:GW5, BQ6, BR6:BR32, BS6, BU6, BW6 +154
ƒ195–238 44 similar groups (401 cells; same shape, refs/constants differ) e.g. E6:AH6 → =sum(E7:E19); others at AM6:BM6, EA6, EI6, EM6, EP6, EV6 +37
ƒ239–290 52 similar groups (1425 cells; same shape, refs/constants differ) e.g. E7:AH7 → =sumproduct($B63:$B65,E63:E65); others at AM7:BM7, E8:AH8, AM8:BM8, E9:AH9, AM9:BM9, E10:AH10 +45
ƒ291–510 220 similar groups (2139 cells; same shape, refs/constants differ) e.g. BQ7:BQ20 ↓ =AR7/BQ$6; others at BS7:BS20, BU7:BU20, BW7:BW20, BX7:BX20, BZ7:BZ20, CB7:CB20 +213
ƒ511–516 6 similar groups (28 cells; same shape, refs/constants differ) e.g. FP7:FP18 ↓ =FM7*0.35; others at FP20, FP24, FP36:FP47, FP48, FP52
ƒ517 FU7:FU20 ↓ =0.35*FS7
ƒ518 FU24 =0.3*FS24
ƒ519–521 3 similar groups (348 cells; same shape, refs/constants differ) e.g. R21:AH23 ▦ =R223+R226; others at AM21:BM23, AM25:BM32
ƒ522–529 8 similar groups (108 cells; same shape, refs/constants differ) e.g. W25:AA29 ▦ =W259; others at AB25:AD30, AE25:AH32, EA62:EA69, C267:C274, C285:C292, C301:C308 +1
ƒ530–538 9 similar groups (21 cells; same shape, refs/constants differ) e.g. EC36  =43.8%*EB36; others at EC37, EC38, EC39, EC40:EC48, EC53, EC54:EC57 +2
ƒ539–540 FV36:FV48 ↓ =FW36/($FW$35+$FW$48+$FW$52) — same formula in 2 ranges (14 cells): FV52; breaks at FV49:FV51 = 3 blank
ƒ541–721 181 similar groups (12018 cells; same shape, refs/constants differ) e.g. E63:AH65 ▦ =sumifs('Item Sales (POS)'!7:7,'Item Sales (POS)'!$2:$2,E$2,'Item Sales (POS)'!$1:$1,E$1); others at AM63:BM65, E66:AH68, AM66:BM68, E69:AH71, AM69:BM71, E72:AH74 +174
ƒ722–723 X105:AH105 → =X109+X112+X117+X120+X123+X126+X129+X132 — same formula in 2 ranges (38 cells): AM105:BM105
ƒ724–727 4 similar groups (40 cells; same shape, refs/constants differ) e.g. GM277:GO281 ▦ =X277*$GL277; others at GS277:GS281, GM293:GO297, GS293:GS297
typed arithmetic: 1 cells hold ref-free formulas (hand-entered sums) e.g. `=1868` in columns AF
◆ formatting
bg#D9EAD3 (17121 cells): B106:B141, B145:AH146, B150:AH150, B155:AH156 +35
bg#FFF2CC (15530 cells): B63:W105, B161:AH162, B182:AH187, B223:AH225 +29
bg#F4CCCC (4606 cells): B20:AH20, B24:AH24, B48:AH48, B52:AH52 +168
bg#00FF00 (4446 cells): AI4:AL61, AI63:AL105, AI107:AI246, AI248:AI257 +3
bg#EAD1DC (4095 cells): B196:AH198, B207:AH208, B236:AH238, B244:AH244 +15
bg#D0E0E3 (4092 cells): B199:AH201, B209:AH210, B216:AH217, B232:AH234 +11
bg#FCE5CD (4070 cells): B163:AH164, B166:AH175, B202:AH204, B226:AH228 +9
bg#CFE2F3 (1458 cells): C338:C349, C352:C355, C362:C365, C373:C376 +7
+18 more styles

## 4. Monthly Summary (Usage) · C1:BZ249 · 11k cells · frozen 4r4c
reads: Sales (6g), Item Sales (Cal) (8g), Item Sales (POS) (1g), Fish record (16g), Salmon (4g), Tuna (4g), Chicken (14g), Chicken BB (1g), Mango (8g), Avocado (8g), MahachanokMango (8g), Pomelo (14g), Tamago (14g), Edamame (14g), Ikura (14g), Others (36g)
▸ C1:BZ19 row-oriented 19×76 (1392 cells), labels in column C
  1 "Year": ƒ1; num 2022 … 2028 (74 filled)
  2 "Month": ƒ2; num 1 … 12 (74 filled)
  3 "Date": ƒ3; date 1/31/2022 … 2/29/2028 (74 filled)
  4 "Average daily sale": ƒ4; num 0.00 … 36,561.58 fmt #,##0.00; 8 errors (67 filled)
  5 "TCM ยอดขายเฉพาะPOKE": ƒ5,ƒ6; num 0.00 … 1,133,409.00 fmt #,##0.00; 8 errors (72 filled)
  6 "instore": ƒ7,ƒ8; num 0.00 … 433,813.00 fmt #,##0.00; 8 errors (72 filled)
  7 "Delivery": ƒ9,ƒ10; num 0.00 … 887,239.00 fmt #,##0.00; 8 errors (72 filled)
  8 "แซลมอน (กรัม/1000บาท)": ƒ136; num 67.37 ก. … 152.27 ก. fmt #,##0.00 ก.;[Red](#,##0.00); 16 errors (74 filled)
  9 "แซลมอนทั้งหมด": ƒ11,ƒ12; num 0.00 … 151,178.00 fmt #,##0.00 (74 filled)
  10 "Yield": ƒ147; num 87.15% … 92.25% fmt 0.00%; 16 errors (72 filled)
  11 "waste": ƒ13,ƒ14; num 0 … 15791 (72 filled)
  12 "closed": ƒ15,ƒ16; num 0 … 9137 (72 filled)
  13 "expired": ƒ154,ƒ155; num 0 (72 filled)
  14 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ156; num 0.00 … 24,870.00 fmt #,##0.00 (72 filled)
  15 "ปริมาณทิ้งทั้งหมด (%)": ƒ163; num 9.63% … 22.14% fmt 0.00%; 16 errors (72 filled)
  16 "ใบนับของสด-ใบจดปลา (กรัม)": ƒ17,ƒ18; num 0.00 … 5,735.00 fmt #,##0.00 (72 filled)
  17 "ใบนับของสด-ใบจดปลา (%)": ƒ164; num 1.02% … 11.10% fmt 0.00%; 16 errors (72 filled)
  18 "ใบจดปลา-Item Sales(กรัม)": ƒ19,ƒ20; num 0.00 … 76,325.00 fmt #,##0.00 (72 filled)
  19 "ใบจดปลา-Item Sales (%)": ƒ180; num 2.84% … 100.00% fmt 0.00%; 16 errors (72 filled)
▸ G21:BZ27 table 7×72 (504 cells), no header row
  G…AK (31 alike): ƒ21,ƒ187,ƒ195 +4; num 131.59 ก. … 119,445.00 fmt #,##0.00 (7 filled)
  AL -: ƒ22,ƒ187,ƒ195 +4; num 0.00 fmt #,##0.00; 4 errors (7 filled)
  AM -: ƒ22,ƒ187,ƒ195 +4; num 0.00 fmt #,##0.00; 4 errors (7 filled)
  AN -: ƒ22,ƒ188,ƒ195 +4; num 0.00 fmt #,##0.00; 4 errors (7 filled)
  AO…BJ (22 alike): ƒ22,ƒ188,ƒ195 +4; num 128.11 ก. … 74,205.00 fmt #,##0.00 (7 filled)
  BK…BZ (16 alike): ƒ22,ƒ188,ƒ195 +4; num 0.00 fmt #,##0.00; 4 errors (7 filled)
  sample (G…V of 72):
  21| 119,445.00 | 113,535.00 | 73,785.00 | 70,080.00 | 64,170.00 | 96,450.00 | 86,160.00 | 99,166.00 | 96,100.00 | 102,556.00 | 113,655.00 | 67,291.00 | 84,400.00 | 72,563.00 | 80,394.00 | 75,058.00
  22| 158.28 ก. | 160.66 ก. | 106.19 ก. | 104.02 ก. | 105.43 ก. | 131.77 ก. | 140.54 ก. | 149.20 ก. | 148.70 ก. | 146.66 ก. | 146.14 ก. | 117.23 ก. | 108.18 ก. | 105.41 ก. | 112.59 ก. | 108.04 ก.
  23| 42,352.84 | 41,808.09 | 29,825.97 | 27,290.91 | 29,337.36 | 41,783.73 | 35,716.89 | 43,322.62 | 38,958.10 | 50,880.37 | 53,786.09 | 30,699.14 | 36,926.23 | 30,490.85 | 29,389.15 | 34,197.35
  27| 16051.59 | 13006.68 | 8224.04 | 9445.59 | 10337.43 | 8344.75 | 9501.41 | 9544.50 | 10140.46 | 10327.97 | 10700.60 | 10788.70 | 9355.82 | 7242.88 | 7096.17 | 6578.93
▸ C29:BZ40 row-oriented 12×76 (880 cells), labels in column C
  29 "ทูน่า (กรัม/1000บาท)": ƒ137; num 33.77 ก. … 56.27 ก. fmt #,##0.00 ก.;[Red](#,##0.00); 16 errors (74 filled)
  30 "ทูน่าทั้งหมด": ƒ25,ƒ26; num 0.00 … 55,142.00 fmt #,##0.00 (74 filled)
  31 "Yield": ƒ148; num 97.93% … 100.00% fmt 0.00%; 16 errors (72 filled)
  32 "waste": ƒ27,ƒ28; num 0 … 992 (72 filled)
  33 "closed": ƒ29,ƒ30; num 0 … 3212 (72 filled)
  34 "expired": ƒ31,ƒ32; num 0 (72 filled)
  35 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ157; num 0.00 … 3,393.00 fmt #,##0.00 (72 filled)
  36 "ปริมาณทิ้งทั้งหมด (%)": ƒ165; num 1.02% … 9.53% fmt 0.00%; 16 errors (72 filled)
  37 "ใบนับของสด-ใบจดปลา (กรัม)": ƒ33,ƒ34; num 0.00 … 5,945.00 fmt #,##0.00 (72 filled)
  38 "ใบนับของสด-ใบจดปลา (%)": ƒ166; num 5.32% … 19.33% fmt 0.00%; 16 errors (72 filled)
  39 "ใบจดปลา-Item Sales(กรัม)": ƒ35,ƒ36; num 0.00 … 43,665.00 fmt #,##0.00 (72 filled)
  40 "ใบจดปลา-Item Sales (%)": ƒ181; num 1.27% … 100.00% fmt 0.00%; 16 errors (72 filled)
▸ G42:BZ48 table 7×72 (504 cells), no header row
  G…AK (31 alike): ƒ37,ƒ191,ƒ196 +4; num 22.18 ก. … 31,620.00 fmt #,##0.00 (7 filled)
  AL -: ƒ38,ƒ191,ƒ196 +4; num 0.00 fmt #,##0.00; 4 errors (7 filled)
  AM -: ƒ38,ƒ191,ƒ196 +4; num 0.00 fmt #,##0.00; 4 errors (7 filled)
  AN -: ƒ38,ƒ192,ƒ196 +4; num 0.00 fmt #,##0.00; 4 errors (7 filled)
  AO…BJ (22 alike): ƒ38,ƒ192,ƒ196 +4; num 62.69 ก. … 49,425.00 fmt #,##0.00 (7 filled)
  BK…BZ (16 alike): ƒ38,ƒ192,ƒ196 +4; num 0.00 fmt #,##0.00; 4 errors (7 filled)
  sample (G…V of 72):
  42| 31,620.00 | 32,685.00 | 44,640.00 | 41,910.00 | 38,385.00 | 30,120.00 | 28,845.00 | 28,351.00 | 28,990.00 | 29,686.00 | 33,525.00 | 39,946.00 | 52,105.00 | 45,788.00 | 51,684.00 | 44,488.00
  43| 59.74 ก. | 65.61 ก. | 74.51 ก. | 77.51 ก. | 66.89 ก. | 53.54 ก. | 61.16 ก. | 59.51 ก. | 60.46 ก. | 50.46 ก. | 49.78 ก. | 70.44 ก. | 65.83 ก. | 68.32 ก. | 84.84 ก. | 66.93 ก.
  44| 17,442.44 | 18,956.67 | 23,705.06 | 23,624.36 | 22,882.06 | 17,902.39 | 16,876.51 | 18,331.13 | 17,534.85 | 19,620.76 | 20,297.90 | 22,945.02 | 26,128.59 | 22,537.30 | 24,924.94 | 23,554.30
  48| 1521.33 | 879.52 | 1004.54 | 929.77 | 849.36 | 1137.39 | 1131.08 | 1039.06 | 728.25 | 711.00 | 846.30 | 696.28 | 764.26 | 646.42 | 324.11 | 925.57
▸ C50:BZ56 row-oriented 7×76 (515 cells), labels in column C
  50 "มะม่วง (กรัม/1000บาท)": ƒ138; num 0.00 ก. … 146.43 ก. fmt #,##0.00 ก.;[Red](#,##0.00); 16 errors (74 filled)
  51 "มะม่วงทั้งหมด": ƒ41,ƒ42; num 0.00 … 132,100.00 fmt #,##0.00 (74 filled)
  52 "Yield": ƒ201; num 0.00% … 12592.80% fmt 0.00%; 17 errors (72 filled)
  53 "waste ใบของสด": ƒ43,ƒ44; num 0 … 12918 (72 filled)
  54 "expired": ƒ45,ƒ46; num 0 … 1130 (72 filled)
  55 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ47,ƒ48; num -31232 … 102599 fmt #,##0.00 (72 filled)
  56 "ปริมาณทิ้งทั้งหมด (%)": ƒ167; num -12492.80% … 100.00% fmt 0.00%; 17 errors (72 filled)
▸ C59:BZ124 row-oriented 66×76 (4826 cells), labels in column C
  59 "อโวคาโด (กรัม/1000บาท)": ƒ139; num 72.01 ก. … 105.81 ก. fmt #,##0.00 ก.;[Red](#,##0.00); 16 errors (74 filled)
  60 "อโวคาโดทั้งหมด": ƒ49,ƒ50; num 0.00 … 100,618.00 fmt #,##0.00 (74 filled)
  61 "Yield": ƒ202; num 0.00% … 52.82% fmt 0.00%; 16 errors (72 filled)
  62 "waste ใบของสด": ƒ51,ƒ52; num 0 … 24230 (72 filled)
  63 "expired": ƒ53,ƒ54; num 0 … 390 (72 filled)
  64 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ55,ƒ56; num 0 … 92674 (72 filled)
  65 "ปริมาณทิ้งทั้งหมด (%)": ƒ168; num 47.18% … 100.00% fmt 0.00%; 16 errors (72 filled)
  66 "มะม่วงมหาชนก (กรัม/1000บาท)": ƒ140,ƒ141; num 0.00 ก. … 119.76 ก. fmt #,##0.00 ก.;[Red](#,##0.00); 16 errors (72 filled)
  67 "มะม่วงมหาชนกทั้งหมด": ƒ57,ƒ58; num 0.00 … 115,118.00 fmt #,##0.00 (72 filled)
  68 "Yield": ƒ203; num 26.97% … 187.58% fmt 0.00%; 61 errors (72 filled)
  69 "waste ใบของสด": ƒ59,ƒ60; num 0 … 12616 (72 filled)
  70 "expired": ƒ61,ƒ62; num 0 … 110 (72 filled)
  71 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ63,ƒ64; num -37,598.00 … 83,636.00 fmt #,##0.00 (72 filled)
  72 "ปริมาณทิ้งทั้งหมด (%)": ƒ169; num -87.58% … 72.90% fmt 0.00%; 61 errors (72 filled)
  73 "ส้มโอ (กรัม/1000บาท)": ƒ142; num 11.43 ก. … 34.61 ก. fmt #,##0.00 ก.;[Red](#,##0.00); 16 errors (74 filled)
  74 "ส้มโอ ทั้งหมด": ƒ65,ƒ66; num 0.00 … 34,099.00 fmt #,##0.00 (74 filled)
  75 "Yield": ƒ149; num 91.71% … 98.83% fmt 0.00%; 16 errors (72 filled)
  76 "waste ใบของสด": ƒ67,ƒ68; num 0 … 2376 (72 filled)
  77 "expired ใบของสด": ƒ69,ƒ70; num 0 … 2536 (72 filled)
  78 "waste ใบเตรียมของ": ƒ71,ƒ72; num 0.00% … 16200.00% fmt 0.00% (72 filled)
  79 "expired ใบเตรียมของ": ƒ73,ƒ74; num 0 … 2380 (72 filled)
  80 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ158; num 0.00 … 5,074.00 fmt #,##0.00 (72 filled)
  81 "ปริมาณทิ้งทั้งหมด (%)": ƒ170; num 7.04% … 30.27% fmt 0.00%; 16 errors (72 filled)
  82 "ใบนับของสด-ใบเตรียมของ (กรัม)": ƒ75,ƒ76; num 0.00 … 2,376.00 fmt #,##0.00 (72 filled)
  83 "ใบนับของสด-ใบเตรียมของ (%)": ƒ171; num 1.17% … 8.29% fmt 0.00%; 16 errors (72 filled)
  84 "ใช้ไปใบเตรียมของ-Item Sales(ก…": ƒ77,ƒ78; num 0.00 … 16,142.00 fmt #,##0.00 (72 filled)
  85 "ใช้ไปใบเตรียมของ-Item Sales (…": ƒ182; num 4.96% … 101.04% fmt 0.00%; 16 errors (72 filled)
  86 "ถั่วแระ (กรัม/1000บาท)": ƒ143; num 44.88 ก. … 79.52 ก. fmt #,##0.00 ก.;[Red](#,##0.00); 16 errors (72 filled)
  87 "ถั่วแระ ทั้งหมด": ƒ79,ƒ80; num 0.00 … 82,000.00 fmt #,##0.00 (72 filled)
  88 "Yield": ƒ150; num 39.50% … 50.63% fmt 0.00%; 16 errors (72 filled)
  89 "waste ใบของสด": ƒ81,ƒ82; num 0 … 44612 (72 filled)
  90 "expired ใบของสด": ƒ83,ƒ84; num 0 (72 filled)
  91 "waste ใบเตรียมของ": ƒ85,ƒ86; num 0.00% fmt 0.00% (72 filled)
  92 "expired ใบเตรียมของ": ƒ87,ƒ88; num 0 … 1341 (72 filled)
  93 "ปริมาณทิ้งทั้งหมด (กรัม)": ƒ159; num 0.00 … 44,612.00 fmt #,##0.00 (72 filled)
  94 "ปริมาณทิ้งทั้งหมด (%)": ƒ172; num 0.49 … 0.61 fmt #,##0.00; 16 errors (72 filled)
  95 "ใบนับของสด-ใบเตรียมของ (กรัม)": ƒ89,ƒ90; num 0.00 … 44,612.00 fmt #,##0.00 (72 filled)
  96 "ใบนับของสด-ใบเตรียมของ (%)": ƒ173; num 49.37% … 60.50% fmt 0.00%; 16 errors (72 filled)
  97 "ใบเตรียมของ-Item Sales(กรัม)": ƒ91,ƒ92; num 0.00 … 23,604.00 fmt #,##0.00 (72 filled)
  98 "ใบเตรียมของ-Item Sales (%)": ƒ183; num 2.02% … 107.79% fmt 0.00%; 16 errors (72 filled)
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
▸ V126:BZ138 table 13×57 (741 cells), no header row
  V…BJ (41 alike): ƒ146,ƒ121,ƒ153 +10; num 0 … 26,000.00 fmt #,##0.00 (13 filled)
  BK…BZ (16 alike): ƒ146,ƒ122,ƒ153 +10; num 0.00 fmt #,##0.00; 5 errors (13 filled)
  sample (V…AK of 57):
  126| 27.05 ก. | 26.03 ก. | 12.42 ก. | 15.87 ก. | 17.70 ก. | 20.97 ก. | 25.55 ก. | 26.34 ก. | 32.62 ก. | 30.29 ก. | 33.56 ก. | 31.56 ก. | 28.56 ก. | 30.00 ก. | 29.24 ก. | 30.50 ก.
  127| 26,000.00 | 21,000.00 | 11,000.00 | 13,000.00 | 15,000.00 | 18,000.00 | 23,000.00 | 26,000.00 | 29,000.00 | 29,000.00 | 32,000.00 | 33,000.00 | 28,000.00 | 30,000.00 | 29,000.00 | 29,000.00
  128| 100.00% | 100.00% | 100.00% | 99.23% | 101.85% | 97.98% | 103.86% | 103.42% | 107.30% | 111.02% | 107.38% | 106.68% | 104.69% | 108.95% | 109.57% | 104.32%
  138| 0.77% | 0.48% | -0.45% | 2.91% | -0.29% | 1.25% | 3.64% | 2.75% | 2.75% | 2.42% | 2.57% | 2.34% | 4.77% | 1.22% | 0.91% | 76.23%
▸ AT140:BI141 (18 cells)
  140: AT=ƒ135→14,000.00 ก. AU=ƒ135→30,226.00 ก. AV=ƒ135→28,000.00 ก. AW=ƒ135→27,000.00 ก. AX=ƒ135→37,000.00 ก. AY=ƒ135→45,000.00 ก. AZ=ƒ135→45,000.00 ก. BA=ƒ135→45,000.00 ก. BB=ƒ135→42,000.00 ก. BC=ƒ135→50,000.00 ก. BD=ƒ135→20,000.00 ก. BE=ƒ135→18,000.00 ก. BF=ƒ135→18,000.00 ก. BG=ƒ135→14,000.00 ก. BH=ƒ135→21,000.00 ก. BI=ƒ135→16,000.00 ก.
  141: BG=ƒ204→110,000.00 BH=ƒ204→131,000.00
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
▸ G143:BZ160 table 18×72 (1296 cells), no header row
  G…BJ (56 alike): ƒ205,ƒ207,ƒ209 +15; num 0.42 ก. … 23.53 ก. fmt #,##0.00 ก.;[Red](#,##0.00) (18 filled)
  BK…BZ (16 alike): ƒ206,ƒ208,ƒ210 +15; 18 errors (18 filled)
  sample (G…V of 72):
  143| 1.94 ก. | 2.23 ก. | 2.18 ก. | 2.07 ก. | 1.82 ก. | 2.33 ก. | 1.91 ก. | 1.50 ก. | 2.08 ก. | 1.85 ก. | 1.48 ก. | 1.49 ก. | 1.38 ก. | 1.36 ก. | 1.70 ก. | 1.46 ก.
  144| 2.13 ก. | 1.83 ก. | 1.86 ก. | 1.55 ก. | 1.88 ก. | 1.48 ก. | 2.02 ก. | 1.55 ก. | 2.06 ก. | 1.60 ก. | 1.38 ก. | 1.74 ก. | 1.78 ก. | 1.50 ก. | 1.68 ก. | 1.14 ก.
  145| 0.42 ก. | 0.41 ก. | 0.42 ก. | 0.40 ก. | 0.38 ก. | 0.36 ก. | 0.41 ก. | 0.36 ก. | 0.35 ก. | 0.39 ก. | 0.40 ก. | 0.37 ก. | 0.33 ก. | 0.31 ก. | 0.27 ก. | 0.20 ก.
  160| 4.85 ก. | 0.30 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.00 ก. | 0.71 ก. | 0.40 ก. | 0.00 ก. | 0.00 ก.
▸ G167:BZ168 table 2×72 (144 cells), no header row
  G…BJ (56 alike): ƒ241,ƒ242; num 6.71% … 9.07% fmt 0.00% (2 filled)
  BK…BZ (16 alike): ƒ241,ƒ242; 2 errors (2 filled)
  sample (G…V of 72):
  167| 6.71% | 5.57% | 8.27% | 8.14% | 12.00% | 5.06% | 6.77% | 6.18% | 4.20% | 5.64% | 4.47% | 9.94% | 4.42% | 3.14% | 2.83% | 4.27%
  168| 9.07% | 5.56% | 4.43% | 4.68% | 4.99% | 8.35% | 7.89% | 8.82% | 5.33% | 6.21% | 5.80% | 3.81% | 2.77% | 2.58% | 1.13% | 4.09%
▸ +5 smaller blocks (21 cells): C21:C27, C42:C48, C166:C168, C247:C249, C140
ƒ formulas (11k cells, 244 groups, 20 families)
ƒ1 E1:BZ1 → =year(E3)
ƒ2 E2:BZ2 → =month(E3)
ƒ3 AH3:BZ3 → =EOMONTH(AG3,1)
ƒ4 L4:BZ4 → =L5/day(L3) [8 errors]
ƒ5–135 131 similar groups (4591 cells; same shape, refs/constants differ) e.g. G5:AM5 → =sumifs(Sales!$D$3:$D$1000,Sales!$B$3:$B$1000,G$2,Sales!$C$3:$C$1000,G$1); others at AN5:BZ5, G6:AM6, AN6:BZ6, G7:AM7, AN7:BZ7, G9:AM9 +124
ƒ136–146 11 similar groups (705 cells; same shape, refs/constants differ) e.g. G8:BZ8 → =G9/G5*1000 [16 errors]; others at G29:BZ29, G50:BZ50, G59:BZ59, G66:H66, I66:BZ66, G73:BZ73 +4
ƒ147–153 G10:BZ10 → =(G9-G11)/G9 [16 errors] — same formula in 7 ranges (489 cells): G31:BZ31, G75:BZ75, G88:BZ88, G101:BZ101, G114:BZ114, V128:BZ128; breaks at G11:BZ30, G32:BZ74, G76:BZ87, G89:BZ100, G102:BZ113 = 6696 different formulas, 432 blank
ƒ154 G13:AM13 → =sumifs('Fish record'!$G$3:$G$1000,'Fish record'!$B$3:$B$1000,G$2,'Fish record'!$C$3:$C$1000,G$1)*-1
ƒ155 AN13:BZ13 → =sumifs('Fish record'!$G$3:$G$2000,'Fish record'!$B$3:$B$2000,AN$2,'Fish record'!$C$3:$C$2000,AN$1)*-1
ƒ156–162 7 similar groups (489 cells; same shape, refs/constants differ) e.g. G14:BZ14 → =sum(G11:G13); others at G35:BZ35, G80:BZ80, G93:BZ93, G106:BZ106, G119:BZ119, V133:BZ133
ƒ163–179 17 similar groups (1194 cells; same shape, refs/constants differ) e.g. G15:BZ15 → =G14/G9 [16 errors]; others at G17:BZ17, G36:BZ36, G38:BZ38, G56:BZ56, G65:BZ65, G72:BZ72 +10
ƒ180–186 7 similar groups (489 cells; same shape, refs/constants differ) e.g. G19:BZ19 → =G18/(G9-G11-G12) [16 errors]; others at G40:BZ40, G85:BZ85, G98:BZ98, G111:BZ111, G124:BZ124, V138:BZ138
ƒ187–194 8 similar groups (288 cells; same shape, refs/constants differ) e.g. G22:AM22 → =(G23+G24)/G$6*1000 [2 errors]; others at AN22:BZ22, G25:AM25, AN25:BZ25, G43:AM43, AN43:BZ43, G46:AM46 +1
ƒ195–196 G23:BZ23 → =G21-G26 — same formula in 2 ranges (144 cells): G44:BZ44; breaks at G24:BZ43 = 1296 different formulas, 144 blank
ƒ197–200 4 similar groups (288 cells; same shape, refs/constants differ) e.g. G24:BZ24 → =(G23/G21)*G14 [19 errors]; others at G27:BZ27, G45:BZ45, G48:BZ48
ƒ201–203 G52:BZ52 → =(G51-G55-G54)/G51 [17 errors] — same formula in 3 ranges (216 cells): G61:BZ61, G68:BZ68; breaks at G53:BZ60, G62:BZ67 = 864 different formulas, 144 blank
ƒ204 BG141:BH141 → =sum(BG127,BG140)
ƒ205–240 36 similar groups (1296 cells; same shape, refs/constants differ) e.g. G143:AM143 → =SUMIFS(Others!$J$3:$J$1000,Others!$B$3:$B$1000,G$2,Others!$C$3:$C$1000,G$1)/G$5*1000; others at AN143:BZ143, G144:AM144, AN144:BZ144, G145:AM145, AN145:BZ145, G146:AM146 +29
ƒ241 G167:BZ167 → =G12/(G9-G11) [16 errors]
ƒ242 G168:BZ168 → =G33/(G30-G32) [16 errors]
ƒ244 C248:C249 ↓ ='Item Sales (POS)'!C484
typed arithmetic: 1 cells hold ref-free formulas (hand-entered sums) e.g. `=#REF!` in columns C
◆ formatting
fg#434343 italic (5126 cells): C10:BZ10, C14:BZ20, C23:BZ24, C26:C27 +46
italic (2580 cells): C11:BZ13, C32:BZ34, C53:D54, C62:D63 +10
bg#FFF2CC (799 cells): C50:BZ51, C66:BZ67, C73:BZ74, C99:BZ100 +3
bg#D9EAD3 (304 cells): C59:BZ60, C86:BZ87
bg#C9DAF8 (229 cells): C5:BZ7, C142
bg#EFEFEF bold (228 cells): C1:BZ3
bg#FCE5CD (152 cells): C8:BZ9
bg#F9CB9C fg#434343 italic (152 cells): C22:BZ22, C25:BZ25
+7 more styles

## 5. Item Sales (Cal) · A1:BOM1145 · 969k cells · frozen 5r4c
reads: Item Sales (POS) (1308g)
read by: Monthly Summary (Usage), Item Sales (Usage)
▸ A1:BOM656 table 656×1755 (969235 cells), no header row
  A -: ƒ7; num 1 … 654 (654 filled)
  B -: num 24  … 60  fmt #,##0_);[Red](#,##0); enum s(79) t(34) st(18) (139 filled)
  C -: text 451 distinct e.g. "Chipotle K-Mix (FPD onl…", "Hunter’s Bowl - พิเศษ", "Hunter’s Bowl - สแตนดาร…" (614 filled)
  D -: num (60) … 200 fmt #,##0_);[Red](#,##0); text "g" (330 filled)
  E…AIN (920 alike): ƒ1,ƒ4,ƒ8 +114; date Sun,  ,  … 1-Mar-22; num (24) … 3510; 6 errors (494 filled)
  AIO…AIV (8 alike): ƒ1,ƒ4,ƒ6 +1; date Fri,  ,  … 6-Sep-24; num 9 … 2024 (4 filled)
  AIW…BOM (823 alike): ƒ1,ƒ4,ƒ6 +172; date Sat,  ,  … 14-Sep-24; num (240) … 2270; 6 errors (569 filled)
  sample (A…P of 1755):
  1|  |  |  |  | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022
  2|  |  |  |  | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3
  3| 1 |  | วันที่ |  | 1-Mar-22 | 2-Mar-22 | 3-Mar-22 | 4-Mar-22 | 5-Mar-22 | 6-Mar-22 | 7-Mar-22 | 8-Mar-22 | 9-Mar-22 | 10-Mar-22 | 11-Mar-22 | 12-Mar-22
  656| 654 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
▸ HU1145 (1 cells)
  1145: HU=ƒ621→0
ƒ formulas (968k cells, 1387 groups, 22 families)
ƒ1–3 E1:ANH1 → =year(E3) — same formula in 3 ranges (1752 cells): ANI1:ANI2, ANJ1:BOM1; breaks at ANI1 = 1 different formulas
ƒ4–5 E2:ANH2 → =month(E3) — same formula in 2 ranges (1750 cells): ANJ2:BOM2; breaks at ANI2 = 1 different formulas
ƒ6 F3:BOM3 → =E3+1
ƒ7 A6:A656 ↓ =A5+1
ƒ8 E4:BOM4 → =day(E3)
ƒ9–621 613 similar groups (734256 cells; same shape, refs/constants differ) e.g. E6:AIL48 ▦ ='Item Sales (POS)'!E6*$D6; others at AIM6:AIM48, AIN6:AIN48, AIW6:AIZ30, AJA6:AJA30, AJB6:AJB30, AJC6:ANA30 +606
ƒ622–1037 416 similar groups (135176 cells; same shape, refs/constants differ) e.g. VL68:AIL68 → =('Item Sales (POS)'!VL103+'Item Sales (POS)'!VL95)*$D68; others at AIM68, AIN68, AIW68:AIZ68, AJA68, AJB68, AJC68:ANA68 +409
ƒ1038–1047 E79:AIN80 ▦ =#REF!*$D79 [#REF!] [1840 errors] — same formula in 10 ranges (10458 cells): AIW79:BOM80, E124:AIN124, AIW124:BOM124, E132:AIN132, AIW132:BOM132, E271:AIN271 +3; breaks at E81:AIN123, E125:AIN131, E133:AIN270, E272:AIN274, AIW81:BOM123 +3 = 271338 different formulas, 61575 blank
ƒ1048–1095 48 similar groups (14038 cells; same shape, refs/constants differ) e.g. AHE196:AHQ198 ▦ =('Item Sales (POS)'!AHE409)*$D196; others at AHR196:AIL201, AIM196:AIM201, AIN196:AIN201, AIW196:AIZ201, AJA196:AJA201, AJB196:AJB201 +41
ƒ1096–1165 70 similar groups (13101 cells; same shape, refs/constants differ) e.g. VL203:AIL203 → =('Item Sales (POS)'!VL247+'Item Sales (POS)'!VL295+'Item Sales (POS)'!VL325)*$D203; others at AIM203, AIN203, AIW203:AIZ203, AJA203, AJB203, AJC203:ANA203 +63
ƒ1166–1167 E444:AIN446 ▦ =sumifs(E$6:E$440,$B$6:$B$440,$B444) — same formula in 2 ranges (5229 cells): AIW444:BOM446
ƒ1168–1171 4 similar groups (3486 cells; same shape, refs/constants differ) e.g. E447:AIN447 → =E444+E446; others at AIW447:BOM447, E448:AIN448, AIW448:BOM448
ƒ1172–1215 44 similar groups (11744 cells; same shape, refs/constants differ) e.g. E451:AD451 → =sum(E452:E470); others at AE451:DF451, DG451:EP451, EQ451:IY451, IZ451:AIN451, AIW451:AXC451, AXD451:BOM451 +37
ƒ1216–1223 8 similar groups (1743 cells; same shape, refs/constants differ) e.g. E481:AIL481 → =('Item Sales (POS)'!E610+ 'Item Sales (POS)'!E165)*$D481; others at AIM481, AIN481, AIW481:AIZ481, AJA481, AJB481, AJC481:ANA481 +1
ƒ1224–1256 33 similar groups (5334 cells; same shape, refs/constants differ) e.g. OS515:AIL515 → =('Item Sales (POS)'!OS6*$D515); others at AIM515, AIN515, AIW515:AIZ515, AJA515, AJB515, AJC515:ANA515 +26
ƒ1257–1296 40 similar groups (6395 cells; same shape, refs/constants differ) e.g. GG525:AIL525 → =('Item Sales (POS)'!GG46+'Item Sales (POS)'!GG378+'Item Sales (POS)'!GG213+'Item Sales (POS)'!GG398)*$D525; others at AIM525, AIN525, AIW525:AIZ525, AJA525, AJB525, AJC525:ANA525 +33
ƒ1297–1326 30 similar groups (3412 cells; same shape, refs/constants differ) e.g. AHE530:AIL530 → =('Item Sales (POS)'!AHE100+'Item Sales (POS)'!AHE108+'Item Sales (POS)'!AHE252+'Item Sales (POS)'!AHE303+'Item Sales (POS)'!AHE332)*$D530; others at AIM530, AIN530, AIW530:AIZ530, AJA530, AJB530, AJC530:ANA530 +23
ƒ1327–1355 29 similar groups (4167 cells; same shape, refs/constants differ) e.g. TI601:TI603 ↓ =('Item Sales (POS)'!TI90+'Item Sales (POS)'!TI336+'Item Sales (POS)'!TI239+'Item Sales (POS)'!TI319+'Item Sales (POS)'!TI402+'Item Sales (POS)'!TI290)*$D601; others at TL601:AIL603, AIM601:AIM603, AIN601:AIN603, AIW601:AIZ603, AJA601:AJA603, AJB601:AJB603 +22
ƒ1356–1360 5 similar groups (2685 cells; same shape, refs/constants differ) e.g. ATW601:BOM601 → =('Item Sales (POS)'!ATW90+'Item Sales (POS)'!ATW336+'Item Sales (POS)'!ATW239+'Item Sales (POS)'!ATW290+'Item Sales (POS)'!ATW319+'Item Sales (POS)'!ATW402+'Item Sales (POS)'!ATW440)*$D601; others at ATW602:BOM602, ATW603:BOM603, ATW604:BOM604, ATW606:BOM606
ƒ1361–1365 5 similar groups (555 cells; same shape, refs/constants differ) e.g. ATE630:ATV630 → =Sum(ATE631:ATE635); others at ATW630:AXC630, AXD630:BHN630, BHO630, BHP630:BOM630
ƒ1366–1369 4 similar groups (3486 cells; same shape, refs/constants differ) e.g. E642:AIN642 → =iferror(sumifs(E$155:E$192,$B$155:$B$192,$B642)+E645+(E644/2),0); others at AIW642:BOM642, E643:AIN643, AIW643:BOM643
ƒ1370 E644:AIN644 → =sumifs(E$155:E$192,$B$155:$B$192,$B644)+E647
ƒ1371 AIW644:BOM644 → =sumifs(AIW$155:AIW$192,$B$155:$B$192,$B644)+AIO647
ƒ1372–1387 16 similar groups (3486 cells; same shape, refs/constants differ) e.g. E645:AIL645 → =sumifs(E$222:E$241,$B$222:$B$241,$B645) * ('Item Sales (POS)'!E381/('Item Sales (POS)'!E381+'Item Sales (POS)'!E88)) [3 errors]; others at AIM645, AIN645, AIW645:AIZ645, AJA645, AJB645, AJC645:ANA645 +9
◆ formatting
bg#FFF2CC (261373 cells): A1:A629, A631:A1145, B6:AIN9, B43:AIN45 +24
bg#EFEFEF (178065 cells): B206:AIN219, B362:AIN440, E194:AIN202, AIW194:BOM202 +2
bg#F9CB9C (123966 cells): B10:AIN13, B40:AIN42, B154:AIN154, B184:AIN185 +16
bg#D9EAD3 (60567 cells): B34:AIN36, B68:D68, B164:C165, B178:AIN180 +17
bg#F4CCCC (54751 cells): A630:BOM630, B1:C4, B18:AIN21, B51:AIN51 +26
bg#CFE2F3 (34920 cells): B14:AIN17, B70:AIN82, B172:AIN174, AIW14:BOM17 +2
bg#FCE5CD (22691 cells): B22:AIN25, B155, B158:D160, B263:AIN264 +7
bg#FFE599 (15722 cells): B46:AIN48, B52:AIN52, B169:AIN171, B533:AIN533 +10
+8 more styles
hidden rows: 71-85, 93-154, 208-218, 270-276, 278-361, 363-439
hidden cols: F-KX, KZ-YY, ZA-AII, AIK-BBB

## 6. Item Sales (POS) · A1:BPC916 · 997k cells · frozen 4r4c
read by: Monthly Summary (item sales), Monthly Summary (Usage), Item Sales (Cal)
▸ A1 (1 cells)
  1: A=ิ
▸ C1:BPC4 table 4×1769 (7069 cells), no header row
  C -: C3=วันที่
  E…BPC (1767 alike): ƒ1,ƒ2; date 1-Mar-22; num 3 … 2022 (4 filled)
  sample (C…R of 1769):
  1|  |  | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022 | 2022
  2|  |  | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3
  3| วันที่ |  | 1-Mar-22 | 2-Mar-22 | 3-Mar-22 | 4-Mar-22 | 5-Mar-22 | 6-Mar-22 | 7-Mar-22 | 8-Mar-22 | 9-Mar-22 | 10-Mar-22 | 11-Mar-22 | 12-Mar-22 | 13-Mar-22 | 14-Mar-22
  4|  |  | Tue,  ,  | Wed,  ,  | Thu,  ,  | Fri,  ,  | Sat,  ,  | Sun,  ,  | Mon,  ,  | Tue,  ,  | Wed,  ,  | Thu,  ,  | Fri,  ,  | Sat,  ,  | Sun,  ,  | Mon,  , 
▸ A5:BMW916 table 912×1713 (989986 cells), no header row
  A -: ƒ4; num 1 … 912 (912 filled)
  C -: text 752 distinct e.g. "ข้าวสาหร่าย", "ผักดอง", "มะม่วง" (813 filled)
  D -: text 3 distinct e.g. "แทนโดย", "ไม่รับ", "ไม่รับท้อปปิ้ง" (3 filled)
  E…AII (915 alike): num 0 … 52 (450 filled)
  AIJ…ANA (122 alike): ƒ11,ƒ15,ƒ18; num 0 (3 filled)
  ANB…BMK (660 alike): num 0 … 32 (669 filled)
  BML…BMW (12 alike): ƒ12,ƒ15,ƒ18; num 0 (3 filled)
  sample (A…P of 1713):
  5| 1 |  | TCM |  |  |  |  |  |  |  |  |  |  |  |  | 
  6| 2 |  | The Classic เล็ก |  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0
  7| 3 |  | The Classic - สแตนดาร์ด |  | 3 | 7 | 7 | 11 | 13 | 7 | 3 | 7 | 7 | 6 | 13 | 9
  916| 912 |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 
ƒ formulas (11k cells, 18 groups, 9 families)
ƒ1 E1:BPC1 → =year(E3)
ƒ2 E2:BPC2 → =month(E3)
ƒ3 F3:BPC3 → =E3+1
ƒ4 A6:A916 ↓ =A5+1
ƒ8 E768 =('Item Sales (POS)'!E23+E360)*$D768
ƒ9–12 E817:ADQ817 → =E607+E611+E615+E619+E623+E627+E631+E635+E639+E643+E648+E665 — same formula in 4 ranges (1706 cells): ADS817:AEK817, AEM817:BBI817, BBK817:BMW817; breaks at ADR817, AEL817, BBJ817 = 1 hardcoded values, 2 blank
ƒ13–15 E818:ADQ818 → =E608+E612+E616+E620+E624+E628+E632+E636+E640+E644+E649 — same formula in 3 ranges (1707 cells): ADS818:AEK818, AEM818:BMW818; breaks at ADR818, AEL818 = 2 blank
ƒ16–18 E819:ADQ819 → =E646 — same formula in 3 ranges (1707 cells): ADS819:AEK819, AEM819:BMW819; breaks at ADR819, AEL819 = 2 blank
typed arithmetic: 3 cells hold ref-free formulas (hand-entered sums) e.g. `=6+1` `=11-1` `=13-1` in columns CA,CB
◆ formatting
bg#EFEFEF (302128 cells): B484:B506, B531:B536, B541:B549, B714:ANF815 +14
bg#FFF2CC (298043 cells): A1:A471, A483:A916, B6:ANF9, B43:ANF45 +48
bg#D9EAD3 (213807 cells): B34:ANF36, B93:ANF122, B230:ANF260, B312:ANF332 +28
bg#F4CCCC (189062 cells): B1:C4, B18:ANF21, B199:ANF200, B208 +44
bg#F9CB9C (116622 cells): B10:ANF13, B40:ANF42, B372:ANF374, B550:ANF605 +12
bg#FFE599 (102486 cells): B46:ANF86, B209:ANF213, B224:ANF229, B360:ANF362 +16
bg#EAD1DC (86583 cells): B87:ANF88, B266:ANF311, B381:ANF381, ANH87:ANM88 +8
bg#CFE2F3 (53010 cells): B14:ANF17, B123:ANF145, B354:ANF356, ANH14:ANM17 +8
+20 more styles
hidden cols: F-KX, KZ-YY, ZA-AIH, AIJ-ANA, ANC-BBB

## 7. Item Sales (Usage) · A2:M2000 · 23k cells · frozen 2r0c
reads: Item Sales (Cal) (10g)
read by: Salmon, Tuna, Chicken, Chicken BB, Mango, Avocado, MahachanokMango, Pomelo, Tamago, Edamame, Ikura
▸ A2:M2000 table 1999×13 (23290 cells), header row 2
  A "รายการ": date Tue, Mar 1, 2022 … Wed, Aug 18, 2027; text "Date" (1998 filled)
  B -: ƒ1; num 1 … 12; text "Month" (1998 filled)
  C -: ƒ2; num 2022 … 2027; text "Year" (1998 filled)
  D "Salmon": ƒ3; num 0 … 5430 (1901 filled)
  E "Tuna": ƒ4; num 0 … 2745 (1893 filled)
  F "Mango": ƒ5; num 0  … 2,266  (1893 filled)
  G "Avocado": ƒ6; num 0 … 2148 (1893 filled)
  H "Pomelo": ƒ7; num 0  … 1,340  (1893 filled)
  I "Tamago": ƒ8; num 0  … 2,064  (1893 filled)
  J "Edamame": ƒ9; num 0  … 1,562  (1893 filled)
  K "Ikura": ƒ10; num 0 … 489 (1893 filled)
  L "Chicken": ƒ11; num 0 … 5680 (1436 filled)
  M "ChickenBB": ƒ12; num 0 … 3380 (697 filled)
  sample (A…M):
  3| Date | Month | Year |  |  |  |  |  |  |  |  |  | 
  4| Tue, Mar 1, 2022 | 3 | 2022 | 3510 | 750 | 1,408  | 1116 | 800  | 1,150  | 928  | 172 |  | 
  5| Wed, Mar 2, 2022 | 3 | 2022 | 4095 | 885 | 1,188  | 1284 | 920  | 1,518  | 968  | 361 |  | 
  2000| Wed, Aug 18, 2027 | 8 | 2027 |  |  |  |  |  |  |  |  |  | 
ƒ formulas (21k cells, 12 groups, 3 families)
ƒ1 B4:B2000 ↓ =month(A4)
ƒ2 C4:C2000 ↓ =year(A4)
ƒ3–12 10 similar groups (17285 cells; same shape, refs/constants differ) e.g. D4:D1904 ↓ =hlookup($A4,'Item Sales (Cal)'!$3:645,'Item Sales (Cal)'!A$447,1); others at E4:E1896, F4:F1896, G4:G1896, H4:H1896, I4:I1896, J4:J1896 +3
◆ formatting
bg#FFF2CC (5994 cells): F3:F2000, H3:H2000, L3:L2000
bg#D9EAD3 (5994 cells): G3:G2000, J3:J2000, M3:M2000
bg#F9CB9C (1998 cells): D3:D2000
bg#F4CCCC (1998 cells): E3:E2000
bg#FFE599 (1998 cells): I3:I2000
bg#FCE5CD (1998 cells): K3:K2000
bold (3 cells): A2:C2
bg#FFF2CC bold (3 cells): F2, H2, L2
+6 more styles

## 8. Monthly Inventory Summary · A1:AE35 · 941 cells
reads: Salmon (14g), Tuna (14g), Mango (7g), Avocado (7g), MahachanokMango (7g), Pomelo (14g), Tamago (14g), Edamame (14g), Eel (14g), Ikura (14g), Others (259g)
▸ A1:AE33 table 33×31 (933 cells), header row 2
  A "รหัส": num 1001 … 3002 (26 filled)
  B "TCM": text 31 distinct e.g. "Avocado", "Edamame", "Eel" (31 filled)
  C "ราคาต่อกรัม": ƒ414,ƒ415,ƒ416 +13; num 0.02 … 180.00 fmt 0.00 (26 filled)
  D "คงเหลือ ⏎Raw material": ƒ1,ƒ8,ƒ15 +28; num 0 … 5690 (31 filled)
  E "คงเหลือ ⏎Ready to use": ƒ51,ƒ65,ƒ79 +20; num 0 … 1845 (23 filled)
  F "คงเหลือ⏎Total": ƒ379,ƒ407,ƒ386; num 0 … 5690 (31 filled)
  G "[31-Dec-2022] มูลค่า": ƒ400; num 0.00 … 6,731.25 fmt #,##0.00;(#,##0.00) (31 filled)
  H "คงเหลือ ⏎Raw material": ƒ2,ƒ9,ƒ16 +28; num 0 … 5360 (31 filled)
  I "คงเหลือ ⏎Ready to use": ƒ53,ƒ67,ƒ81 +20; num 0 … 2067 (23 filled)
  J "คงเหลือ⏎Total": ƒ380,ƒ408,ƒ387; num 0 … 5360 (31 filled)
  K "[31-Jan-2023] มูลค่า": ƒ401; num 0 … 5298.75 (31 filled)
  L "คงเหลือ ⏎Raw material": ƒ3,ƒ10,ƒ17 +28; num 0 … 8500 (31 filled)
  M "คงเหลือ ⏎Ready to use": ƒ55,ƒ69,ƒ83 +20; num 0 … 1860 (23 filled)
  N "คงเหลือ⏎Total": ƒ381,ƒ409,ƒ388; num 0 … 8645 (31 filled)
  O "[28-Feb-2023] มูลค่า": ƒ402; num 0 … 8505 (31 filled)
  P "คงเหลือ ⏎Raw material": ƒ4,ƒ11,ƒ18 +28; num 0 … 11840 (31 filled)
  Q "คงเหลือ ⏎Ready to use": ƒ57,ƒ71,ƒ85 +20; num 0 … 1887 (23 filled)
  R "คงเหลือ⏎Total": ƒ382,ƒ410,ƒ389; num 0 … 11840 (31 filled)
  S "[31-Mar-2023] มูลค่า": ƒ403; num 0 … 10286.25 (31 filled)
  T "คงเหลือ ⏎Raw material": ƒ5,ƒ12,ƒ19 +28; num 0 … 8250 (31 filled)
  U "คงเหลือ ⏎Ready to use": ƒ59,ƒ73,ƒ87 +20; num 0 … 2505 (23 filled)
  V "คงเหลือ⏎Total": ƒ383,ƒ411,ƒ390; num 0 … 8250 (31 filled)
  W "[30-Apr-2023] มูลค่า": ƒ404; num 0 … 5115 (31 filled)
  X "คงเหลือ ⏎Raw material": ƒ6,ƒ13,ƒ20 +28; num 0 … 9970 (31 filled)
  Y "คงเหลือ ⏎Ready to use": ƒ61,ƒ75,ƒ89 +20; num 0 … 2662 (23 filled)
  Z "คงเหลือ⏎Total": ƒ384,ƒ412,ƒ391; num 0 … 9970 (31 filled)
  AA "[31-May-2023] มูลค่า": ƒ405; num 0 … 11508.75 (31 filled)
  AB "คงเหลือ ⏎Raw material": ƒ7,ƒ14,ƒ21 +28; num 0 … 10110 (31 filled)
  AC "คงเหลือ ⏎Ready to use": ƒ63,ƒ77,ƒ91 +20; num 0 … 1705 (23 filled)
  AD "คงเหลือ⏎Total": ƒ385,ƒ413,ƒ392; num 0 … 10110 (31 filled)
  AE "[30-Jun-2023] มูลค่า": ƒ406; num 0 … 6161.25 (31 filled)
  sample (A…P of 31):
  3| 1002 | Salmon Proton | 0.62 | 5030 |  | 5030 | 3,118.60 | 0 |  | 0 | 0 | 5190 |  | 5190 | 3217.8 | 11840
  4| 1001 | Salmon | 0.60 | 2110 |  | 2110 | 1,266.00 | 5360 |  | 5360 | 3216 | 780 |  | 780 | 468 | 2918
  5| 1101 | Tuna (Frozen) | 0.50 | 0 |  | 0 | 0.00 | 0 |  | 0 | 0 | 0 |  | 0 | 0 | 3820
  33| 3002 | ข้าว | 180.00 | 13 |  | 13 | 2,340.00 | 7 |  | 7 | 1260 | 11 |  | 11 | 1980 | 11
▸ B35 (1 cells)
  35: B=Total
▸ G35 (1 cells)
  35: G=ƒ393→25,609.13
▸ K35 (1 cells)
  35: K=ƒ394→16,830.84
▸ O35 (1 cells)
  35: O=ƒ395→21,101.31
▸ S35 (1 cells)
  35: S=ƒ396→34,245.48
▸ W35 (1 cells)
  35: W=ƒ397→22,815.12
▸ AA35 (1 cells)
  35: AA=ƒ398→32,641.88
▸ AE35 (1 cells)
  35: AE=ƒ399→31,621.20
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

## 9. Inventory Summary · A1:I34 · 117 cells · frozen 0r1c
reads: Fish record (6g), Salmon (3g), Tuna (2g), Mango (3g), Avocado (3g), MahachanokMango (1g), Pomelo (2g), Tamago (5g), Edamame (5g), Eel (2g), Ikura (2g), Others (42g)
▸ A1:D2 (5 cells)
  1: A=Date B=ƒ1→10/23/2026
  2: A=TCM C=คงเหลือ Raw materials  … D=คงเหลือ (Beginning Bala…
▸ G1:I5 (9 cells)
  1: G=ƒ2→10/22/2026
  2: G=Waste : Production H=Expired
  3: H=Production I=Ready to use
  4: H=ƒ4→
  5: G=ƒ6→132 H=ƒ7→0 I=ƒ8→200
▸ A4:D34 table 31×4 (85 cells), no header row
  A -: text 31 distinct e.g. "Avocado", "Edamame", "Eel" (31 filled)
  C -: ƒ3,ƒ5,ƒ9 +28; num 0 … 11500 (31 filled)
  D -: ƒ22,ƒ24,ƒ29 +20; num 8 … 2036 (23 filled)
  sample (A…D):
  4| Salmon Proton |  |  | 
  5| Salmon |  | 4015 | 
  6| Tuna (Frozen) |  |  | 
  34| ข้าว |  |  | 
▸ G7:I8 (5 cells)
  7: G=ƒ11→0 H=ƒ12→0 I=ƒ13→210
  8: G=ƒ15→224 H=ƒ16→
▸ G10:H10 (2 cells)
  10: G=ƒ19→ H=ƒ20→
▸ G12:I13 (6 cells)
  12: G=ƒ25→ H=ƒ26→ I=ƒ27→
  13: G=ƒ30→ H=ƒ31→ I=ƒ32→
▸ I20 (1 cells)
  20: I=ƒ47→
▸ G22 (1 cells)
  22: G=ƒ52→162
▸ I25 (1 cells)
  25: I=ƒ59→
▸ G26 (1 cells)
  26: G=ƒ62→
▸ I28 (1 cells)
  28: I=ƒ67→50
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

## 10. Fish record · A1:W1927 · 27k cells · frozen 2r3c
read by: Monthly Summary (Usage), Inventory Summary, Salmon, Tuna
▸ A1:W1927 table 1927×23 (27144 cells), header row 2
  A "[1] Date": date Tue 01-03-2022  … Mon 31-05-2027  (1918 filled)
  B "[2] Month": ƒ2; num 1 … 12 (1918 filled)
  C "[3] Year": ƒ3; num 2022 … 2027 (1918 filled)
  D "[4] Salmon Total Weight": num 0 … 11023 (1721 filled)
  E "[5] Salmon Waste": num 0 … 2142 (1721 filled)
  F "[6] Salmon Waste (Closed)": num 0 … 574 (1721 filled)
  G "[7] Salmon Waste (expired)": num 0 (1721 filled)
  H "[8] Salmon Net Weight": ƒ5; num 0 … 9899 fmt 0 (1918 filled)
  I "[9] Salmon Yield": ƒ7; num 53.72% … 96.87% fmt 0.00%; 228 errors (1925 filled)
  J "[10] Tuna Total Weight": num 0 … 3036 (1721 filled)
  K "[11] Tuna Waste": num 0 … 225 (1721 filled)
  L "[12] Tuna Waste (Closed)": num 0 … 409 (1621 filled)
  M "[13] Tuna Waste (expired)": num 0 (1721 filled)
  N "[14] Tuna Net Weight": ƒ6; num 0 … 3036 (1917 filled)
  O "[15] Tuna Yield": ƒ8; num 84.69% … 100.00% fmt 0.00%; 225 errors (1917 filled)
  P "[16] TCM": empty
  Q "[17] $/g": empty
  R "[18] Discount": empty
  S "[19] Refund": empty
  T "[20] Non Poke Revenue": empty
  U "[21] $ excl. non poke": empty
  V "[22] $/g excl. non poke": empty
  W "Total Waste (closed)": empty
  sample (A…P of 23):
  3| Tue 01-03-2022  | 3 | 2022 | 4361 | 346 | 505 | 0 | 4015 | 92.07% | 838 | 0 | 75 | 0 | 838 | 100.00% | 
  4| Wed 02-03-2022  | 3 | 2022 | 5372 | 600 | 259 | 0 | 4772 | 88.83% | 1001 | 0 | 117 | 0 | 1001 | 100.00% | 
  5| Thu 03-03-2022  | 3 | 2022 | 4911 | 455 | 385 | 0 | 4456 | 90.74% | 928 | 0 | 0 | 0 | 928 | 100.00% | 
  1927|  |  |  |  |  |  |  |  | #DIV/0! Function DIVIDE… |  |  |  |  |  |  | 
ƒ formulas (12k cells, 9 groups, 7 families)
ƒ1 B1:V1 → =A1+1
ƒ2 B3:B1920 ↓ =month(A3)
ƒ3 C3:C1920 ↓ =Year(A3)
ƒ4 D3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=871256527","Fish record summary!D3:g2000") [external] [array/spills]
ƒ5–6 H3:H1920 ↓ =D3-E3 — same formula in 2 ranges (3835 cells): N3:N1919
ƒ7–8 I3:I1927 ↓ =H3/D3 [228 errors] — same formula in 2 ranges (3842 cells): O3:O1919
ƒ9 J3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=871256527","Fish record summary!j3:m2000") [external] [array/spills]
◆ formatting
fg#0000FF (9625 cells): A3:A1927, H3:I1927, N3:O1927
italic (20 cells): A1:G1, J1:V1
bg#D9D9D9 bold underline (7 cells): P2:V2
bg#F4CCCC bold underline (6 cells): J2:O2
bold underline (4 cells): A2:C2, W2
bg#FCE5CD bold underline (4 cells): D2:G2
fg#0000FF italic (2 cells): H1:I1
bg#FCE5CD fg#0000FF bold underline (2 cells): H2:I2
+1 more styles

## 11. Salmon · A1:V2001 · 36k cells · frozen 3r3c
reads: Sales (2g), Item Sales (Usage) (1g), Fish record (5g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
▸ A1:V2001 table 2001×22 (36226 cells), header row 3
  A "[1 / รายการ] Date": date Tue, Mar 1, 2022 … Thu, Aug 19, 2027 (1998 filled)
  B "[2] Month": ƒ4; num 1 … 12 (1998 filled)
  C "[3] Year": ƒ5; num 2022 … 2027 (1998 filled)
  D "[Salmon (กรัม)] Salmon (กรัม/1000บาท)": ƒ6,ƒ7; num 42.40 … 301.08 fmt 0.00; 301 errors (1998 filled)
  E "[4 / 1/1] คงเหลือ": num 0 … 12430 (1698 filled)
  F "[5] ของเข้า(+)": num 870 … 12521 (1486 filled)
  G "[6] ของออก(-)": num 80 … 5600 (25 filled)
  H "[7] ไม่ได้คุณภาพ(-)": num 60 … 1112 (5 filled)
  I "[8] หมดอายุ(-)": empty
  J "[9] ใช้ไป (ใบของสด)": num 0 … 11330 (1998 filled)
  K "[10] ใช้ไป(ใบจดปลา) ก่อน waste": ƒ9,ƒ22,ƒ13; num 0 … 11023 (1998 filled)
  L "[11 ] (ใบนับของสด-ใบจดปลา)": ƒ14; num (283) … 4,015  fmt #,##0_);[Red](#,##0) (1998 filled)
  M "[12 ] % diff": ƒ15; num -10.84% … 21.79% fmt 0.00%; 301 errors (1998 filled)
  N "[13 ] Check ใบของสด": ƒ17; enum Correct(1635) Re-check(62); 301 errors (1998 filled)
  O "[14 ] ใช้ไป(ใบจดปลา) หลัง waste": ƒ10; num 0 … 9899 (1998 filled)
  P "[15 ] waste ปิดร้าน (-)": ƒ11; num 0 … 574 (1998 filled)
  Q "[16 ] waste หมดอายุ (-)": ƒ11; num 0 (1998 filled)
  R "[17 ] item sales": ƒ12; num 0 … 5430 (1998 filled)
  S "[18 ] ใบจดปลา-Item sales": ƒ19,ƒ23,ƒ20; num (1,148) … 7,749  fmt #,##0_);[Red](#,##0) (1998 filled)
  T "[19 ] % diff": ƒ16; num -38.92% … 369.00% fmt 0.00%; 415 errors (1998 filled)
  U "[20 ] Check item sales": ƒ18; enum Correct(1519) Re-check(64); 415 errors (1998 filled)
  V "[21 ] คงเหลือ Proton": num 0 … 19360 (998 filled)
  sample (A…P of 22):
  4| Tue, Mar 1, 2022 | 3 | 2022 | 142.14 | 6047 | 4560 |  |  |  | 4657 | 4361 | 296  | 6.79% | Correct | 4015 | 505
  5| Wed, Mar 2, 2022 | 3 | 2022 | 154.20 | 5950 | 4494 |  |  |  | 5380 | 5372 | 8  | 0.15% | Correct | 4772 | 259
  6| Thu, Mar 3, 2022 | 3 | 2022 | 173.25 | 5064 | 5310 |  |  |  | 5064 | 4911 | 153  | 3.12% | Correct | 4456 | 385
  2001| Thu, Aug 19, 2027 | 8 | 2027 | #DIV/0! Function DIVIDE… |  |  |  |  |  | 0 |  | 0  | #DIV/0! Function DIVIDE… | #DIV/0! Function DIVIDE… |  | 
ƒ formulas (28k cells, 23 groups, 13 families)
ƒ1–3 3 similar groups (20 cells; same shape, refs/constants differ) e.g. B1:C1 → =A1+1; others at E1, F1:V1
ƒ4 B4:B2001 ↓ =month(A4)
ƒ5 C4:C2001 ↓ =year(A4)
ƒ6 D4:D2000 ↓ =K4/Sales!$D4*1000 [300 errors]
ƒ7 D2001 =K2001/Sales!$D3001*1000 [1 errors]
ƒ8 E4 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","1/1!c3:h2000") [external] [array/spills]
ƒ9–13 5 similar groups (9989 cells; same shape, refs/constants differ) e.g. K4:K197 ↓ ='Fish record'!D3; others at O4:O2001, P4:Q2001, R4:R2001, K199:K2001
ƒ14 L4:L2001 ↓ =J4-K4
ƒ15–16 M4:M2001 ↓ =L4/K4 [301 errors] — same formula in 2 ranges (3996 cells): T4:T2001; breaks at N4:S2001 = 11988 different formulas
ƒ17 N4:N2001 ↓ =if(and(M4<10%,M4>-5%),"Correct","Re-check") [301 errors]
ƒ18 U4:U2001 ↓ =if(and(T4<13%,T4>-5%),"Correct","Re-check") [415 errors]
ƒ19–20 S4:S197 ↓ =O4-P4-Q4-R4 — same formula in 2 ranges (1997 cells): S199:S2001; breaks at S198 = 1 different formulas
ƒ21 V4 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","1/1!k3:k1000") [external] [array/spills]
ƒ22 K198 ='Fish record'!D197-2250
ƒ23 S198 =O198-P198-Q198-R198-2250
◐ conditional formats
N2:N2001, U4:U2001: TEXT_EQ Correct → bg#B7E1CD
N2:N2001, U4:U2001: TEXT_EQ Re-check → bg#EA9999
A1:A2001: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#EFEFEF (17982 cells): L4:Q2001, S4:U2001
bg#FCE5CD (17086 cells): D4:D2001, E5:E2001, F4:K2001, V5:V1107
bg#D9EAD3 fg#0000FF (1998 cells): R4:R2001
italic (22 cells): A1:V1
bg#FCE5CD bold (18 cells): D2:K3, V2:V3
bg#EFEFEF bold (18 cells): L2:Q3, S2:U3
bold (3 cells): A2:C2
bold underline (3 cells): A3:C3
+3 more styles
hidden rows: 4-308, 311-674, 676-1405

## 12. Tuna · A1:V2000 · 36k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g), Fish record (3g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Salmon" (81% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Tuna"`
blocks: A1:V2000 (35880 cells)
ƒ 28k cells in 17 groups; 2 families not in "Salmon":
ƒ16 V3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","1/2!c3:c1000") [external] [array/spills]
ƒ17 E12 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","1/1!s12:x2000") [external] [array/spills]
labels not in "Salmon": D:"Tuna (กรัม)" D:"Tuna (กรัม/1000บาท)" V:"คงเหลือ Tuna (Frozen)"

## 13. Chicken · A1:V1929 · 28k cells · frozen 2r0c
reads: Sales (1g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage)
▸ A1:V1929 table 1929×22 (28432 cells), header row 2
  A "[รายการ] Date": date Thu, Jun 1, 2023 … Fri, Sep 8, 2028 (1927 filled)
  B "Month": ƒ1; num 1 … 12 (1927 filled)
  C "Year": ƒ2; num 2023 … 2028 (1927 filled)
  D "[Chicken (กรัม)] Chicken (กรัม/1000บาท)": ƒ3; num 0.00 … 1224.65 fmt 0.00; 233 errors (1927 filled)
  E "[9/2] คงเหลือ": num 0 … 12000 (1241 filled)
  F "ของเข้า(+)": num 1000 … 14000 (502 filled)
  G "ของออก(-)": num 1000 … 4000 (4 filled)
  H "ไม่ได้คุณภาพ(-)": num 74 … 406 (35 filled)
  I "หมดอายุ(-)": I768=307
  J "ใช้ไป (ใบของสด)": num 0 … 7000 (1541 filled)
  K "เตรียมได้": num 800 … 6350 (1085 filled)
  L "[5/3] ใช้ไป⏎(ใบเตรียมของ)": num 0 … 5780 (1541 filled)
  M "ไม่ได้คุณภาพ(-)": ƒ7 (1 filled)
  N "หมดอายุ(-)": num 25 … 500 (16 filled)
  O "เปิดร้าน (ใบเตรียมของ)": num 0 … 5190 (1241 filled)
  P "(ใบนับของสด-เตรียมได้)": ƒ10; num (1,004) … 3,000  fmt #,##0_);[Red](#,##0) (1927 filled)
  Q "[95-115%] % diff": ƒ12; num 0.00% … 123.70% fmt 0.00% (1927 filled)
  R "Check ใบของสด": ƒ13; enum Correct(1740) Re-check(187) (1927 filled)
  S "item sales": ƒ14; num 0 … 5680 (1927 filled)
  T "ใบเตรียมของ-Item sales": ƒ11; num (1,177) … 2,035  fmt #,##0_);[Red](#,##0) (1927 filled)
  U "% diff": ƒ15; num -39.60% … 177.32% fmt 0.00%; 808 errors (1927 filled)
  V "Check item sales": ƒ16; enum Correct(984) Re-check(135); 808 errors (1927 filled)
  sample (A…P of 22):
  3| Thu, Jun 1, 2023 | 6 | 2023 | 67.24 |  | 7000 |  |  |  | 1000 | 1000 | 1000 |  |  |  | 0 
  4| Fri, Jun 2, 2023 | 6 | 2023 | 66.66 | 6000 |  |  |  |  | 1000 | 1000 | 1000 |  |  | 0 | 0 
  5| Sat, Jun 3, 2023 | 6 | 2023 | 55.27 | 5000 |  |  |  |  | 1000 | 1000 | 800 |  |  | 0 | 0 
  1929| Fri, Sep 8, 2028 | 9 | 2028 | #N/A |  |  |  |  |  |  |  |  |  |  |  | 0 
ƒ formulas (19k cells, 16 groups, 15 families)
ƒ1 B3:B1929 ↓ =month(A3)
ƒ2 C3:C1929 ↓ =year(A3)
ƒ3 D3:D1929 ↓ =L3/Sales!$G4*1000 [233 errors]
ƒ4 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1421829428","9/2!K460:P2000") [external] [array/spills]
ƒ5 K3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1360451754","5/3!E460:E2000") [external] [array/spills]
ƒ6 L3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1360451754","5/3!J460:J2000") [external] [array/spills]
ƒ7 M3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1360451754","5/3!G460:G2000") [external] [array/spills]
ƒ8 N3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1360451754","5/3!H460:H2000") [external] [array/spills]
ƒ9 O3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1360451754","5/3!C460:C2000") [external] [array/spills]
ƒ10 P3:P1929 ↓ =J3-K3
ƒ11 T3:T1929 ↓ =L3-S3
ƒ12 Q3:Q1929 ↓ =iferror(K3/J3,0)
ƒ13 R3:R1929 ↓ =if(Or(and(Q3<115%,Q3>95%),and(P3=0,Q3=0%)),"Correct","Re-check")
ƒ14 S3:S1929 ↓ ='Item Sales (Usage)'!L461
ƒ15 U3:U1929 ↓ =T3/S3 [808 errors]
ƒ16 V3:V1929 ↓ =if(and(U3<13%,U3>-5%),"Correct","Re-check") [808 errors]
◐ conditional formats
R1:R1929, V3:V1929: TEXT_EQ Correct → bg#B7E1CD
R1:R1929, V3:V1929: TEXT_EQ Re-check → bg#EA9999
A1:A1929: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (23118 cells): D3:D1929, E4:E1929, F3:J1929, K4:O1929
bg#EFEFEF (11562 cells): P3:R1929, T3:V1929
bg#D9EAD3 fg#0000FF (1927 cells): S3:S1929
bg#FCE5CD bold (24 cells): D1:O2
bg#EFEFEF bold (12 cells): P1:R2, T1:V2
bg#F9CB9C (6 cells): E3, K3:O3
bold (3 cells): A1:C1
bold underline (3 cells): A2:C2
+2 more styles
hidden rows: 4-216, 218-947

## 14. Chicken BB · A1:V1929 · 25k cells · frozen 2r3c
reads: Sales (1g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage)
same layout as "Chicken" (89% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Chicken BB"`
blocks: A1:V1929 (25260 cells)
ƒ 19k cells in 17 groups; 6 families not in "Chicken":
ƒ4 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1421829428","9/2!C460:H2000") [external] [array/spills]
ƒ5 K3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1360451754","5/3!N460:N2000") [external] [array/spills]
ƒ6 L3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1360451754","5/3!S460:S2000") [external] [array/spills]
ƒ7 M3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1360451754","5/3!P460:P2000") [external] [array/spills]
ƒ8 N3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1360451754","5/3!Q460:Q2000") [external] [array/spills]
ƒ9 O3 =IMPORTRANGE("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1360451754","5/3!L460:L2000") [external] [array/spills]
labels not in "Chicken": D:"Chicken BB(กรัม)" D:"ChickenBB (กรัม/1000บาท)"

## 15. Mango · A1:N2000 · 21k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
▸ A1:N2000 table 2000×14 (21094 cells), header row 2
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Thu, Aug 19, 2027 (1998 filled)
  B "Month": ƒ1; num 1 … 12 (1998 filled)
  C "Year": ƒ2; num 2022 … 2027 (1998 filled)
  D "[Mango (กรัม)] มะม่วง (กรัม/1000บาท)": ƒ3,ƒ4; num 0.00 … 232.22 fmt 0.00; 301 errors (1998 filled)
  E "[1/2] คงเหลือ": num 0 … 16667 (1697 filled)
  F "ของเข้า(+)": num 1223 … 15205 (693 filled)
  G "ของออก(-)": num 177 … 10330 (19 filled)
  H "ไม่ได้คุณภาพ(-)": num 14 … 4150 (721 filled)
  I "หมดอายุ(-)": num 50 … 297 (13 filled)
  J "ใช้ไป (ใบของสด)": num 0 … 8725 (1998 filled)
  K "item sales": ƒ6,ƒ7; num 0  … 2,266  fmt #,##0_);[Red](#,##0) (1949 filled)
  L "ใช่ไปใบของสด-Item sales": ƒ8; num (1,672) … 8,725  (1998 filled)
  M "[30%] % Yield": ƒ9; num 0.00% … 383.43% fmt 0.00%; 497 errors (1998 filled)
  N "Check item sales": ƒ10; enum Re-check(1002) Correct(499); 497 errors (1998 filled)
  sample (A…N):
  3| Tue, Mar 1, 2022 | 3 | 2022 | 133.95 | 5690 | 7910 |  |  |  | 4110 | 1,408  | 2,702  | 34.26% | Correct
  4| Wed, Mar 2, 2022 | 3 | 2022 | 103.63 | 9490 |  |  |  |  | 3610 | 1,188  | 2,422  | 32.91% | Correct
  5| Thu, Mar 3, 2022 | 3 | 2022 | 123.47 | 5880 | 10530 |  |  |  | 3500 | 1,408  | 2,092  | 40.23% | Correct
  2000| Thu, Aug 19, 2027 | 8 | 2027 | #DIV/0! Function DIVIDE… |  |  |  |  |  | 0 |  | 0  | #DIV/0! Function DIVIDE… | #DIV/0! Function DIVIDE…
ƒ formulas (14k cells, 10 groups, 8 families)
ƒ1 B3:B2000 ↓ =month(A3)
ƒ2 C3:C2000 ↓ =year(A3)
ƒ3 D3:D1999 ↓ =J3/Sales!$D4*1000 [300 errors]
ƒ4 D2000 =J2000/Sales!$D3001*1000 [1 errors]
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","1/2!k3:p2000") [external] [array/spills]
ƒ6–7 K3:K66 ↓ ='Item Sales (Usage)'!F4 — same formula in 2 ranges (1949 cells): K116:K2000
ƒ8 L3:L2000 ↓ =J3-K3
ƒ9 M3:M2000 ↓ =K3/J3 [497 errors]
ƒ10 N3:N2000 ↓ =if(and(M3<100%,M3>30%),"Correct","Re-check") [497 errors]
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
hidden rows: 3-307, 309-672, 674-703, 705-1404

## 16. Avocado · A1:N2000 · 22k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Mango" (77% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Avocado"`
blocks: A1:N2000 (21574 cells)
ƒ 14k cells in 9 groups; 2 families not in "Mango":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","2/1!c3:h2000") [external] [array/spills]
ƒ8 M3:M2000 ↓ =K3/(J3+H3) [300 errors]
labels not in "Mango": D:"Avocado (กรัม)" D:"อโวคาโด (กรัม/1000บาท)"

## 17. MahachanokMango · A1:N2000 · 19k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Mango" (88% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "MahachanokMango"`
blocks: A1:N2000 (18672 cells)
ƒ 14k cells in 13 groups; 2 families not in "Mango":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","1/2!s3:x2000") [external] [array/spills]
ƒ11 N3:N4 ↓ =if(and(M3<13%,M3>-5%),"Correct","Re-check") [2 errors]
labels not in "Mango": D:"Mahachanok Mango (กรัม)"

## 18. Pomelo · A1:V2000 · 32k cells · frozen 2r0c
reads: Sales (2g), Item Sales (Usage) (1g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Chicken" (72% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Pomelo"`
blocks: A1:V2000 (31648 cells)
ƒ 20k cells in 16 groups; 5 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","6/2!c3:h2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","3/1!n3:n2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","3/1!s3:s2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","3/1!p3:q2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","3/1!l3:l2000") [external] [array/spills]
labels not in "Chicken": D:"Pomelo (กรัม)" D:"ส้มโอ (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-เตรียมได้" Q:"% Yield"

## 19. Tamago · A1:V2000 · 32k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Chicken" (70% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Tamago"`
blocks: A1:V2000 (31643 cells)
ƒ 20k cells in 19 groups; 6 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=2057223341","3/2!k3:p2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1923577576","2/1!n3:n2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1923577576","2/1!s3:s2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1923577576","2/1!p3:q2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1923577576","2/1!l3:l2000") [external] [array/spills]
ƒ19 T127 =L127-S127-N127
labels not in "Chicken": D:"Tamago (กรัม)" D:"ไข่หวาน (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-เตรียมได้" Q:"% Yield" V:"-5%-20%"

## 20. Edamame · A1:V2000 · 31k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Chicken" (66% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Edamame"`
blocks: A1:V2000 (31443 cells)
ƒ 20k cells in 17 groups; 6 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","3/1!s3:x2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1923577576","1/2!w3:w2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1923577576","1/2!ab3:ab2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","1/2!y3:z2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","1/2!u3:u2000") [external] [array/spills]
ƒ13 R3:R2000 ↓ =if(Or(and(Q3<=60%,Q3>40%),and(P3=0,Q3=0%)),"Correct","Re-check")
labels not in "Chicken": D:"Edamame (กรัม)" D:"ถั่วแระ (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-เตรียมได้" Q:"% Yield" U:"-5%-20%"

## 21. Eel · A1:R2000 · 26k cells · frozen 2r3c
reads: Sales (2g)
read by: Monthly Inventory Summary, Inventory Summary
▸ A1:R2000 table 2000×18 (26106 cells), header row 2
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Thu, Aug 19, 2027 (1998 filled)
  B "Month": ƒ1; num 1 … 12 (1998 filled)
  C "Year": ƒ2; num 2022 … 2027 (1998 filled)
  D "[ปลาไหล ] ปลาไหล (กรัม/1000บาท)": ƒ3,ƒ4; num -0.08 … 38.28 fmt 0.00; 301 errors (1998 filled)
  E "[2/2] คงเหลือ": num 0 … 6600 (1698 filled)
  F "ของเข้า(+)": num 200 … 4000 (82 filled)
  G "ของออก(-)": G1636=200
  H "ไม่ได้คุณภาพ(-)": H9=25 H751=21
  I "หมดอายุ(-)": empty
  J "ใช้ไป (ใบของสด)": num 0 … 2600 (1998 filled)
  K "[1/1] เตรียมได้": num 0 … 1000 (1285 filled)
  L "ใช้ไป(ใบเตรียมของ) 5g": num -2 … 962 (1998 filled)
  M "เปิดร้าน (ใบเตรียมของ)": num 0 … 410 (1698 filled)
  N "(ใบนับของสด-เตรียมได้": ƒ9; num (220) … 2,600  fmt #,##0_);[Red](#,##0) (1998 filled)
  O "[100%] % Yield": ƒ11; num 0.00% … 210.00% fmt 0.00% (1998 filled)
  P "Check ใบของสด": ƒ12; enum Correct(1288) Re-check(710) (1998 filled)
  Q "[5/2] เตรียมได้(ใบเตรียมของ) 2.5g": num 14 … 980 (1336 filled)
  R "⏎ปลาไหลใช้ไป (5g)            …": ƒ10; num -112 … 480 (1998 filled)
  sample (A…P of 18):
  3| Tue, Mar 1, 2022 | 3 | 2022 | 8.31 | 4600 |  |  |  |  | 200 | 204 | 255 | 201 | (4) | 102.00% | Correct
  4| Wed, Mar 2, 2022 | 3 | 2022 | 4.31 | 4400 |  |  |  |  | 200 | 184 | 150 | 150 | 16  | 92.00% | Re-check
  5| Thu, Mar 3, 2022 | 3 | 2022 | 6.49 | 4200 |  |  |  |  | 200 | 205 | 184 | 184 | (5) | 102.50% | Correct
  2000| Thu, Aug 19, 2027 | 8 | 2027 | #DIV/0! Function DIVIDE… |  |  |  |  |  | 0 |  | 0 |  | 0  | 0.00% | Correct
ƒ formulas (14k cells, 13 groups, 11 families)
ƒ1 B3:B2000 ↓ =month(A3)
ƒ2 C3:C2000 ↓ =year(A3)
ƒ3 D3:D1999 ↓ =L3/Sales!$D4*1000 [300 errors]
ƒ4 D2000 =L2000/Sales!$D3001*1000 [1 errors]
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","2/2!c3:h2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","1/1!e3:e2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","1/1!j3:j2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","1/1!c3:c2000") [external] [array/spills]
ƒ9 N3:N2000 ↓ =J3-K3
ƒ10 R3:R2000 ↓ =L3-Q3
ƒ11 O3:O2000 ↓ =iferror(K3/J3,0)
ƒ12 P3:P2000 ↓ =if(Or(and(O3<105%,O3>95%),and(N3=0,O3=0%)),"Correct","Re-check")
ƒ13 Q3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","5/2!n3:n2000") [external] [array/spills]
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
hidden rows: 3-308, 310-673, 675-1404

## 22. Ikura · A1:V2000 · 30k cells · frozen 2r3c
reads: Sales (2g), Item Sales (Usage) (2g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
same layout as "Chicken" (70% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> "Ikura"`
blocks: A1:V2000 (30190 cells)
ƒ 20k cells in 17 groups; 5 families not in "Chicken":
ƒ5 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=8961612","3/1!k3:p2000") [external] [array/spills]
ƒ6 K3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","1/2!n3:n2000") [external] [array/spills]
ƒ7 L3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","1/2!s3:s2000") [external] [array/spills]
ƒ8 M3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","1/2!p3:q2000") [external] [array/spills]
ƒ9 O3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=381150158","1/2!l3:l2000") [external] [array/spills]
labels not in "Chicken": D:"Ikura (กรัม)" D:"ไข่ปลา (กรัม/1000บาท)" L:"ใช้ไป(ใบเตรียมของ)" P:"(ใบนับของสด-เตรียมได้" Q:"% Yield" U:"-5%-15%"

## 23. Others · A1:HW3000 · 324k cells · frozen 2r1c
reads: Sales (40g)
read by: Monthly Summary (Usage), Monthly Inventory Summary, Inventory Summary
▸ A1:HW3000 table 3000×231 (323957 cells), header row 2
  A "[รายการ] Date": date Tue, Mar 1, 2022 … Wed, May 15, 2030 (2998 filled)
  B "Month": ƒ1; num 1 … 12 (2998 filled)
  C "Year": ƒ2; num 2022 … 2030 (2998 filled)
  D "[Engawa] ครีบปลา (กรัม/1000บาท)": ƒ3,ƒ161,ƒ22 +1; num -0.35 … 9.93 fmt 0.00; 1301 errors (2998 filled)
  E "[2/2] คงเหลือ": num 0 … 1300 (1698 filled)
  F "ของเข้า(+)": num 100 … 1000 (149 filled)
  G "ของออก(-)": empty
  H "ไม่ได้คุณภาพ(-)": num 33 … 76 (11 filled)
  I "หมดอายุ(-)": empty
  J "ใช้ไป (ใบของสด)": num 0 … 500 (1998 filled)
  K "เปิดร้าน (ใบเตรียมของ)": num 0 … 300 (1698 filled)
  L "[1/1] ใช้ไป(ใบเตรียมของ) ": num -11 … 285 (1998 filled)
  M "เตรียมได้": num 24 … 300 (656 filled)
  N "[100%] % Yield": ƒ58; num 0% … 104% fmt 0% (2998 filled)
  O "[Engawa] Check ใบของสด": ƒ78; enum Correct(2867) Re-check(131) (2998 filled)
  P "[ก้ามปู] ก้ามปู (กรัม/1000บาท)": ƒ4,ƒ24; num -1.11 … 8.75 fmt 0.00; 1301 errors (2998 filled)
  Q "[2/2] คงเหลือ": num 0 … 1200 (1698 filled)
  R "ของเข้า(+)": num 100 … 1000 (154 filled)
  S "ของออก(-)": empty
  T "ไม่ได้คุณภาพ(-)": empty
  U "หมดอายุ(-)": empty
  V "ใช้ไป (ใบของสด)": num 0 … 800 (1998 filled)
  W "เปิดร้าน (ใบเตรียมของ)": num 0 … 304 (1698 filled)
  X "[1/1] ใช้ไป(ใบเตรียมของ) ": num -30 … 233 (1998 filled)
  Y "เตรียมได้": num 90 … 300 (693 filled)
  Z "[100%] % Yield": ƒ59; num 0% … 133% fmt 0% (2998 filled)
  AA "[ก้ามปู] Check ใบของสด": ƒ79; enum Correct(2982) Re-check(16) (2998 filled)
  AB "[กุ้ง] กุ้ง (กรัม/1000บาท)": ƒ5,ƒ25; num -0.15 … 1.52 fmt 0.00; 1301 errors (2998 filled)
  AC "[3/1] คงเหลือ": num 0 … 345 (1698 filled)
  AD "ของเข้า(+)": num 10 … 300 (349 filled)
  AE "ของออก(-)": num 10 … 300 (3 filled)
  AF "ไม่ได้คุณภาพ(-)": AF64=1
  AG "หมดอายุ(-)": empty
  AH "ใช้ไป (ใบของสด)": num -20 … 70 (1998 filled)
  AI "เปิดร้าน (ใบเตรียมของ)": num 0 … 38 (1698 filled)
  AJ "[1/2] ใช้ไป(ใบเตรียมของ) ": num -6 … 41 (1998 filled)
  AK "เตรียมได้": num 8 … 300 (994 filled)
  AL "[100%] %Yield": ƒ60,ƒ160,ƒ77; num -50% … 200% fmt 0% (2998 filled)
  AM "[กุ้ง] Check ใบของสด": ƒ80; enum Correct(2986) Re-check(12) (2998 filled)
  AN "[ยำสาหร่าย] ยำสาหร่าย (กรัม/1000บาท)": ƒ6,ƒ26; num 0.00 … 24.40 fmt 0.00; 1301 errors (2998 filled)
  +191 more columns
  sample (A…P of 231):
  3| Tue, Mar 1, 2022 | 3 | 2022 | 2.71 | 500 | 500 |  |  |  | 0 | 298 | 83 |  | 0% | Correct | 1.37
  4| Wed, Mar 2, 2022 | 3 | 2022 | 2.33 | 1000 |  |  |  |  | 150 | 215 | 81 | 150 | 100% | Correct | 2.96
  5| Thu, Mar 3, 2022 | 3 | 2022 | 4.73 | 850 |  |  |  |  | 0 | 284 | 134 |  | 0% | Correct | 1.66
  3000| Wed, May 15, 2030 | 5 | 2030 | #DIV/0! Function DIVIDE… |  |  |  |  |  |  |  |  |  | 0% | Correct | #DIV/0! Function DIVIDE…
ƒ formulas (171k cells, 161 groups, 27 families)
ƒ1 B3:B3000 ↓ =month(A3)
ƒ2 C3:C3000 ↓ =year(A3)
ƒ3–41 39 similar groups (56961 cells; same shape, refs/constants differ) e.g. D3:D1586 ↓ =L3/Sales!$D4*1000; others at P3:P1999, AB3:AB1999, AN3:AN1999, AZ3:AZ1999, BL3:BL1999, BX3:BX1999 +32
ƒ42 E3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1092278103","2/2!K3:P2000") [external] [array/spills]
ƒ43–47 5 similar groups (5 cells; same shape, refs/constants differ) e.g. K3  =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","1/1!l3:l2000") [external] [array/spills]; others at CE3, EA3, FK3, GI3
ƒ48–52 5 similar groups (5 cells; same shape, refs/constants differ) e.g. L3  =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","1/1!s3:s2000") [external] [array/spills]; others at CF3, EB3, FL3, GJ3
ƒ53–57 5 similar groups (5 cells; same shape, refs/constants differ) e.g. M3  =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","1/1!n3:n2000") [external] [array/spills]; others at CG3, EC3, FM3, GK3
ƒ58–77 N3:N3000 ↓ =IFERROR(M3/(J3+H3),0) — same formula in 20 ranges (54027 cells): Z3:Z3000, AL3:AL196, AX3:AX3000, BJ3:BJ3000, BV3:BV3000, CH3:CH3000 +13; breaks at AL197, O3:Y3000, AA3:AW3000, AY3:BI3000, BK3:BU3000 +12 = 139162 hardcoded values, 105005 different formulas, 319458 blank
ƒ78–95 18 similar groups (51030 cells; same shape, refs/constants differ) e.g. O3:O3000 ↓ =if(Or(and(N3<=100%,N3>90%),and(M3=0,N3=0%)),"Correct","Re-check"); others at AA3:AA3000, AM3:AM3000, AY3:AY3000, BK3:BK3000, BW3:BW3000, CI3:CI3000 +11
ƒ96–102 7 similar groups (7 cells; same shape, refs/constants differ) e.g. Q3  =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1092278103","2/2!s3:x2000") [external] [array/spills]; others at BY3, CW3, EG3, FQ3, GO3, HA3
ƒ103–110 8 similar groups (8 cells; same shape, refs/constants differ) e.g. W3  =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","1/1!u3:u2000") [external] [array/spills]; others at BG3, CQ3, DC3, EM3, FW3, GU3 +1
ƒ111–118 8 similar groups (8 cells; same shape, refs/constants differ) e.g. X3  =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","1/1!ab3:ab2000") [external] [array/spills]; others at BH3, CR3, DD3, EN3, FX3, GV3 +1
ƒ119–126 8 similar groups (8 cells; same shape, refs/constants differ) e.g. Y3  =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","1/1!w3:w2000") [external] [array/spills]; others at BI3, CS3, DE3, EO3, FY3, GW3 +1
ƒ127–130 4 similar groups (4 cells; same shape, refs/constants differ) e.g. AC3  =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1092278103","3/1!c3:h2000") [external] [array/spills]; others at AO3, BM3, CK3
ƒ131–135 5 similar groups (5 cells; same shape, refs/constants differ) e.g. AI3  =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","1/2!c3:c2000") [external] [array/spills]; others at AU3, BS3, DO3, EY3
ƒ136–140 5 similar groups (5 cells; same shape, refs/constants differ) e.g. AJ3  =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","1/2!j3:j2000") [external] [array/spills]; others at AV3, BT3, DP3, EZ3
ƒ141–145 5 similar groups (5 cells; same shape, refs/constants differ) e.g. AK3  =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","1/2!e3:e2000") [external] [array/spills]; others at AW3, BU3, DQ3, FA3
ƒ146 BA3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1092278103","3/2!s3:v2000") [external] [array/spills]
ƒ147 BE3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","2/1!z3:z2000") [external] [array/spills]
ƒ148 BF3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1092278103","3/2!x3:x2000") [external] [array/spills]
ƒ149 DI3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1092278103","7/2!c3:f2000") [external] [array/spills]
ƒ150 ES3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1092278103","8/1!c3:f2000") [external] [array/spills]
ƒ151 DM3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","3/2!h3:h1000") [external] [array/spills]
ƒ152 EW3 =importrange("https://docs.google.com/spreadsheets/d/1w9s_ICt8rQtR7WjNi5KOj18hULmVZ_5irRIZG99b-Gs/edit#gid=1940819478","4/1!h3:h2000") [external] [array/spills]
ƒ153 DN3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1092278103","7/2!h3:h1000") [external] [array/spills]
ƒ154 EX3 =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1092278103","8/1!h3:h2000") [external] [array/spills]
ƒ155–158 4 similar groups (4 cells; same shape, refs/constants differ) e.g. DU3  =importrange("https://docs.google.com/spreadsheets/d/1OD0PTAkQLB-v2Omu7xToNUrsH7JsaKoAwbwQ6rZDFsQ/edit#gid=1092278103","7/2!k3:p2000") [external] [array/spills]; others at FE3, GC3, HM3
ƒ159 GA3:GA3000 ↓ =if(Or(and(FZ3<=100%,FZ3>=80%),and(FY3=0,FZ3=0%)),"Correct","Re-check")
ƒ160 AL197 =IFERROR(AK197/(AH197+AE197+AF197),0)
ƒ161 D1587 =IFERROR(L1587/Sales!$D1588*1000, 0)
◐ conditional formats
O1:O3000, AA1:AA3000, AB1 +38: TEXT_EQ Correct → bg#B7E1CD
O1:O3000, AA1:AA3000, AB1 +38: TEXT_EQ Re-check → bg#EA9999
A1:A3000: DATE_EQ YESTERDAY → bg#B7E1CD
◆ formatting
bg#FCE5CD (507574 cells): D3:D3000, E4:E3000, F3:J3000, K4:M3000 +80
bg#EFEFEF (153124 cells): A3:C3000, N3:O3000, Z3:AA3000, AL3:AM3000 +31
bg#FCE5CD bold (352 cells): D1:M2, P1:Y2, AB1:AK2, AN1:AV2 +26
bg#B7E1CD (119 cells): HW3:HW121
bg#F9CB9C (80 cells): E3, K3:M3, Q3, W3:X3 +35
bg#EFEFEF bold (77 cells): A1:C1, N1:O2, Z1:AA2, AL1:AM2 +20
bg#EFEFEF bold underline (3 cells): A2:C2
bg#FF0000 bold (2 cells): DG1, GN1
hidden rows: 4-308, 310-673, 675-1404
hidden cols: DI-DQ, HA-HI, HM-HW
