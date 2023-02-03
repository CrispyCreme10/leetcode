pub mod solution {
    struct TwoSumInput {
        nums: Vec<i32>,
        target: i32
    }

    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num) in nums.iter().enumerate() {
            for (i2, num2) in nums.iter().enumerate() {
                if i == i2 {
                    continue;
                } else if num + num2 == target {
                    return vec![(i as i32), (i2 as i32)];
                }
            }
        }
    
        vec![]
    }

    fn two_sum_opt (nums: Vec<i32>, target: i32) -> Vec<i32> {
        // save old indices o(n)
        let mut nums: Vec<(usize, i32)> = 
            nums.into_iter()
                .enumerate()
                .collect::<Vec<(usize, i32)>>();

        println!("{:?}", nums);

        // sort o(n log n)
        nums.sort_unstable_by_key(|&(_, n)| n);

        println!("{:?}", nums);

        // linear scan for first num o(n)
        for (k, (i, ni)) in nums.iter().enumerate() {
            // binary search o(n log n)
            match nums[k+1..].binary_search_by_key(&(target - *ni), |&(_, nj)| nj) {
                Ok(j) => return vec![*i as i32, nums[j+k+1].0 as i32],
                Err(_) => {}
            }
        }

        // worst case: o(n log n)
        unreachable!("Error: this place should be unreachable");
        return vec![0,0];
    }

    pub fn test_two_sum() {
        let testcases = vec![
            TwoSumInput {
                nums: [2,7,11,15].to_vec(),
                target: 9
            },
            TwoSumInput {
                nums: [3,2,4].to_vec(),
                target: 6
            },
            TwoSumInput {
                nums: [3,3].to_vec(),
                target: 6
            },
        ];
    
        for tc in testcases.iter() {
            let now = std::time::Instant::now();
            let output = two_sum(tc.nums.clone(), tc.target);
            println!("Output : {:?}", output);
            println!("Elapsed: {:.2?}\n", now.elapsed());
        }

        println!("Optimized");

        for tc in testcases.iter() {
            let now = std::time::Instant::now();
            let output = two_sum_opt(tc.nums.clone(), tc.target);
            println!("Output : {:?}", output);
            println!("Elapsed: {:.2?}\n", now.elapsed());
        }
    }
}

