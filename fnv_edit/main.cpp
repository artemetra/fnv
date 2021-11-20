#include <iostream>
#include <fstream>
#include <ios>
#include <string>
#include <iomanip>

#include "Point.hpp"
#include "Curve.hpp"
#include "FileManager.hpp"
#include <filesystem>

void testEveryFnv() {
	//https://stackoverflow.com/a/612176/14296133
	namespace fs = std::filesystem;
	std::string path = "D:\\_FNV\\git\\fnv\\fnv_edit\\_tests\\3\\";
	for (const auto& entry : fs::directory_iterator(path))
		parseFile(entry.path().string());
}

int main() {
	/* just tests */
	//parseFile("3896 points.fnv");
	//parseFile("48k points.fnv");
	//parseFile("fnv.vcxproj.filters");
	//parseFile("drtyogifg.fnv");
	//parseFile("Fuzzy.fnv");
	//parseFile("50 env points.fnv");
	Point bruh = Point(0.1f, 0.2f, Mode::HOLD);
	//testEveryFnv();
	std::cout << sizeof(bruh) << '\n';
	return 0;
}