impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut rest = 0;
        let mut cur = candies;
        let mut v1 = Vec::new();
        for i in 0..num_people{
            v1.push(0);
        }
        let mut cur_idx = 0;
        let mut flag = 1;
        while  flag == 1{
            let mut pos = cur_idx % num_people -1 ;
            if pos == -1 {
                pos = num_people - 1;
            }
            v1[pos] = v1[pos] + cur_idx;


        }

        panic!();
    }
}
struct Solution{

}
fn main() {}