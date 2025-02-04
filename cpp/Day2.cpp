#include "Day2.h"
#include "Utils.h"

#include <cstdio>
#include <string>
#include <iostream>
#include <sstream>
#include <queue>

Day2::Day2()
{
    std::cout << "Day 2" << std::endl;

    std::filesystem::path contentPath = "/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_2.txt";
    const auto lines = Utils::getContentLines (contentPath);

    for (const auto& line : lines)
    {
        std::istringstream stream (line);
        std::string l;

        vector<int> numbers;

        while (getline (stream, l, ' '))
            numbers.push_back (std::stoi (l));

        levels.push_back (numbers);
    }
}

void Day2::runPart1()
{
    int safeLevels = 0;

    for (const auto& level : levels)
        safeLevels += static_cast<int> (sequencePassed (level));

    std::cout << "Safe levels: " << safeLevels << std::endl;
}

void Day2::runPart2()
{
    int safeLevels = 0;
    int index = 0;

    std::queue<int> failedLevels;

    for (const auto& level : levels)
    {
        if (sequencePassed (level))
            ++safeLevels;
        else
            failedLevels.push (index);

        ++index;
    }

    // process failed levels

    auto tryWithoutNumbers = [this] (const std::vector<int>& level) {
        const size_t size = level.size();

        for (int i = 0; i < size; ++i)
        {
            std::vector<int> alteredLevels = level;
            alteredLevels.erase (alteredLevels.begin() + i);

            if (sequencePassed (alteredLevels))
                return true;
        }

        return false;
    };

    for (; ! failedLevels.empty(); failedLevels.pop())
    {
        const int i = failedLevels.front();

        const auto level = levels[i];

        if (tryWithoutNumbers (level))
            ++safeLevels;
    }

    std::cout << "Safe levels: " << safeLevels << std::endl;
}

bool Day2::sequencePassed (const vector<int>& level)
{
    const size_t size = level.size();

    bool isAscending = level[0] < level[1];

    for (int i = 1; i < size; ++i)
    {
        int diff = level[i - 1] - level[i];

        if (diff == 0)
            return false;

        if (diff > 3 || diff < -3)
            return false;

        if (isAscending && level[i] < level[i - 1])
            return false;

        if (! isAscending && level[i] > level[i - 1])
            return false;
    }

    return true;
}