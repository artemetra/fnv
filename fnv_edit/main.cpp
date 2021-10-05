#include <iostream>
#include <fstream>
#include <ios>
#include <string>
#include <iomanip>

#include "Point.hpp"
#include "Curve.hpp"
#include "FileManager.hpp"

int main() {
	
	auto result = read_first_byte("10 points lfo");
	std::cout << result << std::endl;

	return 0;
}