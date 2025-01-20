#include "Utils.h"
#include <fstream>


vector<string> Utils::getContentLines (const filesystem::path& contentPath)
{
    std::ifstream contentFile (contentPath);
    std::string myText;
    vector<string> lines;

    while (getline (contentFile, myText))
        lines.push_back(myText);

    contentFile.close();

    return std::move (lines);
}