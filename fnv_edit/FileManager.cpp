#include <iostream>
#include <fstream>
#include <vector>
#include <limits>
#include <bitset>

#include "Point.hpp"
#include "Curve.hpp"
#include "FileManager.hpp"

//using bits = std::bitset<std::numeric_limits<unsigned char>::digits>;
/*char read_first_byte(const std::string& file_name) {
    if (std::ifstream file{ file_name, std::ios::binary }) {
        file >> std::noskipws;
        char b;
        file >> b;
        return b;
    } else {
        std::cout << "Couldn't open file " << file_name << "! Returning null" << std::endl;
        return '\0';
    }
}*/
void parseFile(const std::string& file_name) {
    if (std::ifstream file{ file_name, std::ios::binary }) {
        CurveType type = getType(file);
        if (type == CurveType::NONE) {
            return;
        } else {
            std::cout << "Parsed fnv is of type ";
        }

    }
}

CurveType getType(std::ifstream& file) {
    char raw_type;
    file.read(reinterpret_cast<char*>(&raw_type), 1);
    switch (raw_type) {
        case 0x01:
            std::cout << "Parsed fnv is of type envelope" << std::endl;
            return CurveType::ENVELOPE;
        case 0x02:
            std::cout << "Parsed fnv is of type LFO" << std::endl;
            return CurveType::LFO;
        case 0x03:
            std::cout << "Parsed fnv is of type graph" << std::endl;
            return CurveType::GRAPH;
        default:
            std::cout << "Invalid file header!" << std::endl;
            return CurveType::NONE;
    }
}