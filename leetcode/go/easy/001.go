func twoSum(nums []int, target int) []int {
    sum_map := make(map[int]int)

    for i, num := range nums {
        remainder := target - num

        if idx, ok := sum_map[remainder]; ok {
            return []int{idx, i}
        }

        sum_map[num] = i
    }

    return nil
}
