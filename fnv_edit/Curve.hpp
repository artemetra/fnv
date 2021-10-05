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
	Curve();
	std::vector<Point> getPoints() const;

};

class EnvelopeCurve : protected Curve {
private:

	CurveType m_type = CurveType::ENVELOPE;
	std::vector<EnvPoint> m_points = {};

	ush m_attack = 0.5f;
	ush m_decay = 0.5f;
	ush m_sustain = 1.0f;
	ush m_release = 0.5f;

	bool m_tempo = false;
	bool m_global = false;
	bool m_isOn = false;

public:
	// TODO: fix this
	EnvelopeCurve(std::vector<EnvPoint> points);
		

	std::vector<EnvPoint> getPoints() const;

	ush getAttack() const;
	ush getDecay() const;
	ush getSustain() const;
	ush getRelease() const;

	bool isTempo() const;
	bool isGlobal() const;
	bool isOn() const;
};

class LfoCurve : protected Curve {
private:

	CurveType m_type = CurveType::LFO;
	std::vector<EnvPoint> m_points = {};

	ush m_speed;
	ush m_lfoTension; // the name is a bit ambiguous with point tension
	ush m_skew;
	ush m_pulsewidth;

	double m_phase;

	bool m_tempo;
	bool m_global;
	bool m_isOn;

public:
	LfoCurve(std::vector<EnvPoint> points);

	std::vector<EnvPoint> getPoints() const;

	ush getSpeed() const;
	ush getLfoTension() const;
	ush getSkew() const;
	ush getPulsewidth() const;
	double getPhase() const;
	bool isTempo() const;
	bool isGlobal() const;
	bool isOn() const;

};

class GraphCurve : protected Curve {
private:
	CurveType m_type = CurveType::GRAPH;
	const unsigned int MIN_POINTS = 2;
};