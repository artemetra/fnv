#pragma once

#include <iostream>
#include <fstream>

#include "Curve.hpp"
#include "Point.hpp"

void parseFile(const std::string& file_name);

CurveType getType(const std::ifstream& file);
uint32_t getSize(const std::ifstream& file);