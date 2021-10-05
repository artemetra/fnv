#include <iostream>
#include <fstream>
#include <vector>
#include <limits>
#include <bitset>

#include "FileManager.h"

#include "Curve.hpp"

using bits = std::bitset<std::numeric_limits<unsigned char>::digits>;
char read_first_byte(const std::string& file_name) {
    if (std::ifstream file{ file_name, std::ios::binary }) {
        file >> std::noskipws;
        char b;
        file >> b;
        return b;
    }
    return '\0';
}
