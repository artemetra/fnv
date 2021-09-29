#pragma once
#include <vector>

#include "Point.h"

enum class CurveType {
	ENVELOPE = 0x01,
	LFO = 0x02,
	GRAPH = 0x03,
};


class Curve {
protected:
	const unsigned int MIN_POINTS = 1;
	CurveType m_type = CurveType::GRAPH;
	std::vector<Point> m_points = {};
public:
	Curve(CurveType curveType, std::vector<Point> points);
};

class EnvelopeCurve : protected Curve {
private:
	
	CurveType m_type = CurveType::ENVELOPE;
	std::vector<EnvPoint> m_points = {};

	ush m_attack;
	ush m_decay;
	ush m_sustain;
	ush m_release;

	bool m_tempo;
	bool m_global;
	bool m_isOn;

public:
	// TODO: fix this
	EnvelopeCurve(std::vector<EnvPoint> points,
		ush attack = 0.5f,
		ush decay = 0.5f,
		ush sustain = 1.0f,
		ush release = 1.0f,
		
	);

};

class LfoCurve : protected Curve {
private:

	CurveType m_type = CurveType::LFO;
	std::vector<EnvPoint> m_points = {};

	ush m_speed;
	ush m_lfoTension; // the name is a bit ambiguous with point tension
	ush m_skew;
	ush m_pulsewidth;

	bool m_tempo;
	bool m_global;
	bool m_isOn;

public:
	LfoCurve(std::vector<EnvPoint> points);

};

class GraphCurve : protected Curve {
private:
	CurveType m_type = CurveType::GRAPH;
	const unsigned int MIN_POINTS = 2;
};