#include <vector>

#include "Curve.h"
#include "Point.h"
#include "Curve.hpp"

Curve::Curve(CurveType curveType, std::vector<Point> points) {
	/* https://t.me/c/1071392087/195908
	bool isEnvelope = (curveType == CurveType::ENVELOPE);
	bool isLFO = (curveType == CurveType::LFO;
	bool has1Point = (points.size() >= 1);
	bool isGraph = (curveType == CurveType::GRAPH);
	bool has2Points = (points.size() >= 2);


	https://t.me/c/1071392087/195922
	This could be more concise, especially if you have more options in the enum:

	bool hasEnoughPoints(CurveType ct, int pNum)
	{
	   switch (ct)
	   {
		case CurveType::ENVELOPE:
		case CurveType::LFO:
		   return pNum >= 2;
		 case CurveType::GRAPH:
		   return pNum >= 1;
		  default:
			   return false;
	   }
	}

	*/


	bool isValid = (
		((curveType == CurveType::ENVELOPE || curveType == CurveType::LFO) && points.size() >= 1) 
		|| (curveType == CurveType::GRAPH && points.size() >= 2)
		);
	
	if (isValid) {
		// Do the actual construction
		m_type = curveType;
		m_points = points;
	}
}

std::vector<Point> Curve::getPoints() const
{	
	return m_points;
}

EnvelopeCurve::EnvelopeCurve(std::vector<Point> points)
	: m_attack(0.5f),
	m_decay(0.5f),
	m_sustain(1.0f),
	m_release(0.5f),
	m_tempo(false),
	m_global(false),
	m_isOn(true)
{
	// do the construction
}