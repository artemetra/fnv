#pragma once
#include <vector>

#include "Point.h"

enum class CurveType {
	ENVELOPE = 0x01,
	LFO = 0x02,
	GRAPH = 0x03,
};


class Curve
{
private:
	CurveType m_type = CurveType::GRAPH;
	std::vector<Point> m_points = {};
public:
	Curve(CurveType curveType, std::vector<Point> points);
};

