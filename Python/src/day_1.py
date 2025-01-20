from src import utils
import numpy as np

class Day1:
    def __init__(self):
        self.file_contents = utils.read_file ("./files/day_1.txt")
        self.left_numbers = []
        self.right_numbers = []

        self.left_numbers, self.right_numbers = self.split_nums(self.file_contents)

    @staticmethod
    def split_nums(content: str):
        line_pair = content.splitlines()

        left_numbers = []
        right_numbers = []

        for n in line_pair:
            left, right = n.split("   ")
            left_numbers.append(int(left))
            right_numbers.append(int(right))
        
        left_numbers.sort()
        right_numbers.sort()

        assert len(left_numbers) == len(right_numbers)
        return left_numbers, right_numbers

    def run_part_1(self):
        diff = []

        for i in range(len(self.left_numbers)):
            diff.append (abs(self.left_numbers[i] - self.right_numbers[i]))

        result = np.sum (diff)
        print (result) 
    
    def run_part_2(self):
        similarity = []
        
        for l_number in self.left_numbers:
            matching_numbers = [x for x in self.right_numbers if x == l_number]
            similarity.append (l_number * len(matching_numbers))
            
        result = np.sum (similarity)
        print (result)
