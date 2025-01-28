from src.day_1 import Day1
from src.day_2 import Day2
from src.day_3 import Day3

def main():
    print ("Run days...")

    # run day one
    day1 = Day1()
    day1.run_part_1()
    day1.run_part_2()

    day2 = Day2()
    day2.run_part_1()
    day2.run_part_2()

    day3 = Day3()
    day3.run_part_1()
    day3.run_part_2()

if __name__ == "__main__":
    main()