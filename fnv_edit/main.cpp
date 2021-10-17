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
	parseFile("3896 points.fnv");
	parseFile("48k points.fnv");
	parseFile("fnv.vcxproj.filters");
	parseFile("drtyogifg.fnv");
	parseFile("Fuzzy.fnv");
	parseFile("50 env points.fnv");
	
	return 0;
}