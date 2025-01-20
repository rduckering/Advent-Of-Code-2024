#pragma once
#include <string>
#include <vector>
#include <filesystem>

using namespace std;

class Utils
{
public:
    static vector<string> getContentLines (const filesystem::path& contentPath);
};
