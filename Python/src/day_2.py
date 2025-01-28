from src import utils

class Day2:
    def __init__(self):
        self.file_contents = utils.read_file ("../files/day_2.txt")
        self.levels = self.parse_content (self.file_contents)

    @staticmethod
    def parse_content(contents):
        lines = contents.splitlines();
        
        levels = []

        for l in lines:
            numbers = l.split(' ')
            converted_numbers = [int(numeric_string) for numeric_string in numbers]
            levels.append(converted_numbers)

        return levels
    
    @staticmethod
    def sequence_passed(level: list):

        is_ascending = level[0] < level[1]

        for i in range(1, len(level)):
            diff = level[i] - level[i - 1]

            if diff == 0 or diff > 3 or diff < -3:
                return False
            
            if is_ascending and level[i] < level[i - 1]:
                return False
            
            if not is_ascending and level[i] > level[i - 1]:
                return False
            
        return True
    
    @staticmethod
    def tryWithoutNumber(level: list):
        for i in range(len(level)):
            altered_numbers = level.copy()
            altered_numbers.pop(i)
            if Day2.sequence_passed (altered_numbers): 
                return True

        return False
    
    def run_part_1(self):
        passed_levels = list(filter(self.sequence_passed, self.levels))
        print(len(passed_levels))

    def run_part_2(self):
        passed_levels = []
        failed_levels = []
        
        for level in self.levels:
            if self.sequence_passed(level):
                passed_levels.append (level)
            else:
                failed_levels.append (level)

        # check through failed levels
        for level in failed_levels:
            if self.tryWithoutNumber(level):
                passed_levels.append(level)

        print(len(passed_levels))