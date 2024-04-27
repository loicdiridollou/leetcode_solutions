"""Problem 0264 - Ugly Number II."""


class Solution:
    """Class for the problem's solution."""

    def nthUglyNumber(self, n: int) -> int:
        """Find n-th ugly number."""
        a, b, c = 0, 0, 0
        lst = [1] * n
        for i in range(1, n):
            lst[i] = min(lst[a] * 2, lst[b] * 3, lst[c] * 5)
            if lst[i] == lst[a] * 2:
                a += 1
            if lst[i] == lst[b] * 3:
                b += 1
            if lst[i] == lst[c] * 5:
                c += 1

        return lst[n - 1]


def test_solution():
    """Test problem solving."""
    sol = Solution()
    assert sol.nthUglyNumber(10) == 12
    assert sol.nthUglyNumber(1) == 1
