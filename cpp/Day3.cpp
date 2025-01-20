#include "Day3.h"
#include "Utils.h"

#include <cstdio>

Day3::Day3()
{
    printf ("Day 3\n");

    std::filesystem::path contentPath = "/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/cpp/files/day_3.txt";
    const auto lines = Utils::getContentLines (contentPath);


    processInput (lines);
    printf("");
}

void Day3::runPart1()
{

}

void Day3::runPart2()
{

}

std::vector<std::string> Day3::processInput (const std::vector<std::string>& contentLines)
{
    std::vector<std::string> instructions;

    std::array<char, 12> scratchBuffer;

    for (const auto& line : contentLines)
    {
        const size_t lineSize = line.size();
        
        for (const auto& c : line)
        {

        }
    }

    return std::move(instructions);
}