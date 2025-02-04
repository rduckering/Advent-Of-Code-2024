from src import utils
import numpy as np

# solution is avoiding using regex!

class Day3:
    def __init__(self):
        self.file_contents = utils.read_file ("../files/day_3.txt")
        self.instructions = self.parse_content (self.file_contents)

    @staticmethod
    def parse_content(contents):
        lines = contents.splitlines()
        
        instructions = []

        for l in lines:
            for i in range(0, len(l), 1):
                buffer = l[i: i + 12]

                if buffer[:4] == "mul(":
                    print ("maybe")

            # for i in l.split(")"):

            #     if "mul" in i:
            #         mul_index = i.find ("mul(")
            #         instructions.append(i[mul_index:] + ')')

            #     elif "don't" in i:
            #         dont_index = i.find ("don't")
            #         instructions.append(i[dont_index:] + ')')

            #     elif "do" in i:
            #         do_index = i.find ("do")
            #         instructions.append(i[do_index:] + ')')

        return instructions
    
    def run_part_1(self):
        sum = 0
        for i in self.instructions:
            if "mul(" in i:
                start_index = i.find("(")
                comma_index = i.find(",")
                end_index = i.find(")")

                first_number = i[start_index + 1: comma_index]
                second_number = i[comma_index + 1: end_index]

                if len(first_number) > 3 or len(first_number) < 1:
                    continue;
                
                if len(second_number) > 3 or len(second_number) < 1:
                    continue;

                sum += int(first_number) * int(second_number)
        
        print(f"Sum: {sum}")

    def run_part_2(self):
        pass
        