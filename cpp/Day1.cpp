#include "Day1.h"

#include <cstdio>
#include <fstream>
#include <iostream>
#include <numeric>

Day1::Day1()
{
    std::cout << "Day 1" << std::endl;

    std::filesystem::path contentPath = "/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_1.txt";

    std::ifstream contentFile (contentPath);

    std::string myText;

    while (getline (contentFile, myText))
    {
        std::string leftNumber = myText.substr(0, myText.find("   "));
        std::string rightNumber = myText.substr(myText.find("   ") + 3, myText.length());

        leftNumbers.push_back (std::stoi (leftNumber));
        rightNumbers.push_back (std::stoi (rightNumber));
    }

    contentFile.close();

    std::sort (leftNumbers.begin(), leftNumbers.end());
    std::sort (rightNumbers.begin(), rightNumbers.end());
}

void Day1::runPart1()
{
    std::vector<int> diff;
    diff.resize (leftNumbers.size());

    for (int i = 0; i < leftNumbers.size(); ++i)
        diff.push_back (abs (leftNumbers[i] - rightNumbers[i]));

    int result = std::accumulate (diff.begin(), diff.end(), 0);

   printf ("Result: %d\n", result);
}

void Day1::runPart2()
{
    std::vector<int> similarity;
    similarity.resize (leftNumbers.size());

    for (const auto& lNum: leftNumbers)
    {
        int cnt = 0;

        for (const auto& rNum: rightNumbers)
        {
            if (lNum == rNum)
                ++cnt;

            // It's sorted so return if the number is largest as they'll be no more.
            if (rNum > lNum)
                break;
        }

        similarity.push_back (lNum * cnt);
    }

    int result = std::accumulate (similarity.begin(), similarity.end(), 0);

    printf ("Result: %d\n", result);
}