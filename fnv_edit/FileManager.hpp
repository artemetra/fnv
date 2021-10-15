#pragma once

#include <iostream>
#include <fstream>

#include "Curve.hpp"
#include "Point.hpp"

void parseFile(const std::string& file_name);

CurveType getType(std::ifstream& file);

//char read_first_byte(const std::string& file_name);