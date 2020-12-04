use std::collections::HashMap;

// --- Day 1: Report Repair ---
//
// After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island.
// Surely, Christmas will go on without you.
//
// The tropical island has its own currency and is entirely cash-only.
// The gold coins used there have a little picture of a starfish; the locals just call them stars.
// None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.
//
// To save your vacation, you need to get all fifty stars by December 25th.
//
// Collect stars by solving puzzles.
// Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first.
// Each puzzle grants one star. Good luck!
//
// Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.
//
// Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
//
// For example, suppose your expense report contained the following:
//
// 1721
// 979
// 366
// 299
// 675
// 1456
//
// In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
//
// Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
fn find_answer_part_1(numbers: &Vec<i32>, target: i32) -> Result<i32, &'static str> {
    let mut seen: HashMap<i32, bool> = HashMap::new();

    for number in numbers.iter() {
        let look = target - *number;
        if seen.contains_key(&look) {
            return Ok(look * number);
        }

        seen.insert(*number, true);
    }

    Err("No answer could be found.")
}

// --- Part Two ---
//
// The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation.
// They offer you a second one if you can find three numbers in your expense report that meet the same criteria.
//
// Using the above example again, the three entries that sum to 2020 are 979, 366, and 675.
// Multiplying them together produces the answer, 241861950.
//
// In your expense report, what is the product of the three entries that sum to 2020?
fn find_answer_part_2(numbers: &Vec<i32>, target: i32) -> Result<i32, &'static str> {
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    for (mut adx, a) in sorted_numbers.iter().enumerate() {
        let mut bdx = adx + 1;
        let mut cdx = sorted_numbers.len() - 1;
        while bdx < cdx {
            let b = sorted_numbers[bdx];
            let c = sorted_numbers[cdx];

            let sum = a + b + c;
            if sum == target {
                return Ok(a * b * c);
            } else if sum < target {
                bdx += 1;
            } else /* if sum > target */ {
                cdx -= 1;
            }
        }

        while *a == sorted_numbers[adx + 1] {
            adx += 1
        }
    }
    
    Err("No answer could be found.")
}

fn main() {
    let target = 2020;

    // let expense_report = vec![1721, 979, 366, 299, 675, 1456];
    let expense_report: Vec<i32> = "
        1227
        1065
        329
        1063
        1889
        1700
        1805
        1373
        389
        1263
        1276
        1136
        1652
        1981
        1406
        1249
        1197
        1379
        1050
        1791
        1703
        2001
        1842
        1707
        1486
        1204
        1821
        1807
        1712
        1871
        1599
        1390
        1219
        1612
        1980
        1857
        1511
        1702
        1455
        1303
        1052
        1754
        1545
        1488
        1848
        1236
        1549
        1887
        1970
        1123
        1686
        1404
        1688
        1106
        1296
        401
        1829
        1693
        1389
        1957
        914
        1176
        1348
        1275
        1624
        1401
        1045
        1396
        1352
        1569
        1060
        1235
        1679
        1503
        1340
        1872
        1410
        1077
        958
        1681
        1189
        1466
        1087
        1852
        1293
        1139
        1300
        1323
        661
        1388
        1983
        1325
        1112
        1774
        1858
        1785
        1616
        1255
        1198
        1354
        1124
        1834
        1417
        1918
        1496
        33
        1150
        1861
        1172
        2006
        1199
        1558
        1919
        1620
        1613
        1710
        1477
        1592
        1709
        1909
        1670
        1922
        1840
        1768
        1982
        1193
        1736
        1877
        1770
        1191
        1433
        1072
        1148
        1225
        1147
        1171
        1424
        1913
        1228
        1339
        1814
        1504
        1251
        1240
        1272
        1500
        1927
        1428
        1641
        1453
        1729
        1976
        1808
        1180
        1024
        1108
        1085
        1669
        1636
        1005
        1520
        1929
        1626
        1551
        1234
        1988
        1256
        1524
        1571
        1506
        1977
        1749
        1408
        1540
        1934
        1810
        1328
        1910
        1478
        1600
        1699
        1413
        1446
        1798
        1013
        1998
        1661
        1058
        1051
        1220
        1447
        1675
        1912
        1668
        1932
        1962
        1055
        1757
        1116
        1090
        "
    .split_whitespace()
    .map(|number_str| number_str.parse())
    .filter_map(Result::ok)
    .collect();

    match find_answer_part_1(&expense_report, target) {
        Ok(v) => println!("The answer to part 1 is: {:?}", v),
        Err(e) => panic!(e),
    }

    match find_answer_part_2(&expense_report, target) {
        Ok(v) => println!("The answer to part 2 is: {:?}", v),
        Err(e) => panic!(e),
    }
}
