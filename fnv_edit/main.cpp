#include <iostream>
#include <fstream>
#include <ios>
#include <string>
#include <iomanip>

#include "Point.hpp"
#include "Curve.hpp"
#include "FileManager.hpp"

int main() {
	/* just tests */
	parseFile("10 points lfo.fnv");
	parseFile("48k points.fnv");
	parseFile("50 env points.fnv");
	return 0;
}