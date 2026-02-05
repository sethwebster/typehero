package main

func maxValue(nums []int) (int, error) {
	if len(nums) == 0 {
		return 0, errors.New("empty slice")
	}
	max := nums[0]
	for _, n := range nums[1:] {
		if n > max {
			max = n
		}
	}
	return max, nil
}
