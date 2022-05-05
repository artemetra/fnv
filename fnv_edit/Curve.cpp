#include <vector>

#include "Curve.hpp"
#include "Point.hpp"

Curve::Curve(const std::vector<Point>& points) {

}
/*Curve::Curve(CurveType curveType, std::vector<Point> points) {
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
/*

	bool isValid = (
		((curveType == CurveType::ENVELOPE || curveType == CurveType::LFO) && points.size() >= 1) 
		|| (curveType == CurveType::GRAPH && points.size() >= 2)
		);
	
	if (isValid) {
		// Do the actual construction
		m_type = curveType;
		m_points = points;
	}
}*/

inline const std::vector<Point>& Curve::vector() const {
	return m_points;
}

inline std::vector<EnvPoint> SharedEnvCurve::vector() const {
	return m_points;
}

inline Point Curve::operator[](const uint32_t& index) const {
	return m_points[index];
}

inline EnvPoint SharedEnvCurve::operator[](const uint32_t index) const {
	return m_points[index];
}


EnvelopeCurve::EnvelopeCurve(std::vector<EnvPoint>& points) {
	// do the construction
}

inline ush_t EnvelopeCurve::getAttack() const {
	return m_attack;
}

inline ush_t EnvelopeCurve::getDecay() const {
	return m_decay;
}

inline ush_t EnvelopeCurve::getSustain() const {
	return m_sustain;
}

inline ush_t EnvelopeCurve::getRelease() const {
	return m_release;
}

inline bool EnvelopeCurve::isTempo() const {
	return m_flags.m_tempo;
}

inline bool EnvelopeCurve::isGlobal() const {
	return m_flags.m_global;
}

inline bool EnvelopeCurve::isOn() const {
	return m_flags.m_on;
}
