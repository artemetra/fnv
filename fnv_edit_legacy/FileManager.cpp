﻿#include <iostream>
#include <fstream>
#include <vector>
#include <limits>
#include <bitset>
#include <stdint.h>

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

// https://stackoverflow.com/a/2072890/14296133
inline bool ends_with(std::string const& value, std::string const& ending) {
    return std::equal(ending.rbegin(), ending.rend(), value.rbegin());
}

void skipBytes(std::ifstream& file, size_t chunks) {
    uint32_t bytes;
    for (int i = 0; i != chunks; i++) {
        file.read(reinterpret_cast<char*>(&bytes), 4);
        std::cout << std::hex << "\t - Read 4 bytes at "
            << file.tellg() << "-" << (uint64_t(file.tellg()) + 4)
            << ": " << std::dec << bytes << std::endl;
    }
    //file.seekg(uint64_t(uint64_t(file.tellg()) + amt));
}

static void readFile(std::ifstream& file) {
    CurveType type = getType(file);
    // expect these since i haven't figured out what the all the bytes mean yet :')
    skipBytes(file, 1);
    uint32_t numberOfPoints = getSize(file);
}

void parseFile(const std::string& file_name) {
    std::cout << "\nReading file \"" << file_name << "\"..." << std::endl;
    if (ends_with(file_name, ".fnv")) {
        if (std::ifstream file{ file_name, std::ios::binary }) {
            readFile(file);
        } else {
            std::cerr << "Couldn't open file " << file_name << "!" << std::endl;
        }
    } else {
        std::cerr << "File passed (" << file_name << ") is not an fnv file" << std::endl;
    }
}





CurveType getType(std::ifstream& file) {
    uint32_t raw_type;
    file.read(reinterpret_cast<char*>(&raw_type), 4);
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
        case 0x07:
            std::cout << "Parsed fnv is of type map" << std::endl;
            return CurveType::MAP;
            
        default:
            std::cout << "Invalid file header!" << std::endl;
            return CurveType::NONE;
    }
}

uint32_t getSize(std::ifstream& file) {
    uint32_t size;
    file.read(reinterpret_cast<char*>(&size), 4);
    std::cout << "The curve has " << size << " points" << std::endl;
    return size;
}