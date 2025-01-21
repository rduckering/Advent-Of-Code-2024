#include "Day3.h"
#include "Utils.h"

#include <cstdio>
#include <string>

Day3::Day3()
{
    printf ("Day 3\n");

    std::filesystem::path contentPath = "/Users/reubenduckering/Documents/Personal Repo/Advent-Of-Code-2024/files/day_3.txt";
    const auto lines = Utils::getContentLines (contentPath);

    processInput (lines, validInstructions);

    printf("");
}

void Day3::runPart1()
{
    int sum = 0;

    for (const auto& instruction :  validInstructions)
    {
        if (instruction.starts_with ("mul("))
        {
            const size_t startIndex = instruction.find ('(');
            const size_t seperatorIndex = instruction.find (',');
            const size_t endIndex = instruction.find (')');

            const std::string firstNumber = instruction.substr (startIndex + 1, seperatorIndex - startIndex - 1);
            const std::string endNumber   = instruction.substr (seperatorIndex + 1, endIndex - seperatorIndex - 1);

            sum += std::stoi (firstNumber) * std::stoi (endNumber);
        }
    }

    printf("Sum: %i\n", sum);
}

void Day3::runPart2()
{
    int sum = 0;

    bool sumArguments = true;

    for (const auto& instruction :  validInstructions)
    {

        if (instruction.starts_with ("don't"))
        {
            sumArguments = false;
        }
        else if (instruction.starts_with ("do"))
        {
            sumArguments = true;
        }

        if (! sumArguments)
            continue;

        if (instruction.starts_with ("mul("))
        {
            const size_t startIndex = instruction.find ('(');
            const size_t seperatorIndex = instruction.find (',');
            const size_t endIndex = instruction.find (')');

            const std::string firstNumber = instruction.substr (startIndex + 1, seperatorIndex - startIndex - 1);
            const std::string endNumber   = instruction.substr (seperatorIndex + 1, endIndex - seperatorIndex - 1);

            sum += std::stoi (firstNumber) * std::stoi (endNumber);
        }
    }

    printf("Sum: %i\n", sum);
}

bool isWord (const std::array<char, 12>& buffer, const char* word)
{
    if (std::memcmp (&buffer, word, sizeof (char) * strlen (word)) == 0)
        return true;

    return false;
}

void Day3::processInput (const std::vector<std::string>& contentLines, std::vector<std::string>& instructions)
{
    std::array<char, 12> scratchBuffer;

    for (const auto& line : contentLines)
    {
        int valid_character_count = 0;

        for (const auto& c : line)
        {
            if (c == 'm')
            {
                valid_character_count = 0;
                scratchBuffer[valid_character_count] = c;
                continue;
            }

            if (c == 'd')
            {
                valid_character_count = 0;
                scratchBuffer[valid_character_count] = c;
                continue;
            }

            if (valid_character_count >= scratchBuffer.size() - 1)
                continue;

            scratchBuffer[++valid_character_count] = c;

            // end of sequence
            if (c == ')')
            {
                if (isWord (scratchBuffer, "don't"))
                {
                    instructions.emplace_back ("don't");
                }
                else if (isWord (scratchBuffer, "do"))
                {
                    instructions.emplace_back ("do");
                }
                else if (isWord (scratchBuffer, "mul("))
                {
                    char a[valid_character_count + 1];

                    std::memcpy (&a, &scratchBuffer, sizeof(char) * valid_character_count + 1);
                    const std::string valueToCheck = std::string (a);

                    const size_t startIndex = valueToCheck.find ('(');
                    const size_t commaIndex = valueToCheck.find (',');
                    const size_t endIndex = valueToCheck.find (')');

                    if (startIndex > endIndex)
                        continue;

                    if (commaIndex == -1)
                        continue;

                    if (commaIndex - startIndex > 4 || commaIndex - startIndex <= 1)
                        continue;

                    if (endIndex - commaIndex > 4 || endIndex - commaIndex <= 1)
                        continue;

                    instructions.emplace_back (valueToCheck);
                    valid_character_count = 0;
                }
            }
        }
    }
}