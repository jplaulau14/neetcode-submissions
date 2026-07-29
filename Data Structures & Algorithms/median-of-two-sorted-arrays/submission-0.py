class Solution:
    def findMedianSortedArrays(
        self,
        nums1: list[int],
        nums2: list[int],
    ) -> float:
        if len(nums1) > len(nums2):
            return self.findMedianSortedArrays(nums2, nums1)

        short_length = len(nums1)
        long_length = len(nums2)
        left_size = (short_length + long_length + 1) // 2
        low = 0
        high = short_length

        while low <= high:
            short_cut = low + (high - low) // 2
            long_cut = left_size - short_cut

            short_left = (
                float("-inf")
                if short_cut == 0
                else nums1[short_cut - 1]
            )
            short_right = (
                float("inf")
                if short_cut == short_length
                else nums1[short_cut]
            )
            long_left = (
                float("-inf")
                if long_cut == 0
                else nums2[long_cut - 1]
            )
            long_right = (
                float("inf")
                if long_cut == long_length
                else nums2[long_cut]
            )

            if short_left <= long_right and long_left <= short_right:
                if (short_length + long_length) % 2 == 1:
                    return float(max(short_left, long_left))

                return (
                    max(short_left, long_left)
                    + min(short_right, long_right)
                ) / 2.0

            if short_left > long_right:
                high = short_cut - 1
            else:
                low = short_cut + 1

        raise RuntimeError("unreachable")