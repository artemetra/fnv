#include <iostream>
#include <fstream>
#include <ios>
#include <string>
#include <iomanip>
#include <filesystem> // std::filesystem
#include <memory>

#include "Point.hpp"
#include "Curve.hpp"
#include "FileManager.hpp"

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
	//std::shared_ptr<Curve> cur = std::make_shared<Curve>();
	//PointDefArgs pointDefaultParams;
	//Point point = Point(cur, 1, 1, pointDefaultParams);
	//testEveryFnv();
	//std::cout << sizeof(point) << '\n';
	return 0;
}