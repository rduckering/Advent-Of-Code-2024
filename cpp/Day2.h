#pragma once
#include <vector>

class Day2
{
public:
    Day2();
    ~Day2() = default;

    void runPart1();
    void runPart2();

private:

    static bool sequencePassed (const std::vector<int>& level);

    std::vector<std::vector<int>> levels;
};

