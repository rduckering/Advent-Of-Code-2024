import unittest
from src.day_1 import Day1

class TestDay1(unittest.TestCase):
    def test_split_numbers(self):
        left_numbers, right_numbers = Day1.split_nums("12   15\n14   19")
        self.assertEqual(left_numbers[0],12)
        self.assertEqual(left_numbers[1], 14)
        self.assertEqual(right_numbers[0], 15)
        self.assertEqual(right_numbers[1], 19)

if __name__ == "__main__":
    unittest.main()