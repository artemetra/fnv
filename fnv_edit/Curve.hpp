#pragma once
#include <vector>

#include "Point.hpp"

enum class CurveType {
	NONE, // only internal, not actually used in files
	ENVELOPE = 0x01,
	LFO = 0x02,
	GRAPH = 0x03,
	MAP = 0x07
};

/*struct ADSR_Section
{
	uint32_t decay = false;
	bool loopStart = false;
	bool sustain = false;
	bool* loopEnd = &sustain;
	// in fl, it's written as "Sustain / Loop End" so i
	// might use either of them
};*/

class Curve {
protected:
	const unsigned int MIN_POINTS = 1;
	CurveType m_type = CurveType::GRAPH;
	std::vector<Point> m_points = {};
public:
	Curve(const std::vector<Point>& points);
	inline const std::vector<Point>& vector() const;
	inline Point operator[](const uint32_t& index) const;
};

class GraphCurve : protected Curve {
protected:
	CurveType m_type = CurveType::GRAPH;
	const unsigned int MIN_POINTS = 2;
public:
	GraphCurve(std::vector<Point>& points);
};

class MapCurve : protected GraphCurve {
public:
	MapCurve(std::vector<Point>& points);
};

struct Flags {
	bool m_tempo = false;
	bool m_global = false;
	bool m_on = false;
};

struct ADSR_Section {
	// default values, meaning that no point in the curve
	// actually has these values
	uint32_t decay = 0xFFFFFFFF;
	uint32_t loopStart = 0xFFFFFFFF;
	uint32_t sustain = 0xFFFFFFFF;

	uint32_t* loopEnd = &sustain;
	// in fl, it's written as "Sustain / Loop End" so i
	// might use either of them
};

class SharedEnvCurve {
protected:
	const unsigned int MIN_POINTS = 1;
	
	std::vector<EnvPoint> m_points = {};

	ADSR_Section m_adsrPoints;
	Flags m_flags;

public:
	inline std::vector<EnvPoint> vector() const;
	inline EnvPoint operator[](const uint32_t index) const;
};

class EnvelopeCurve : protected SharedEnvCurve {
private:
	CurveType m_type = CurveType::ENVELOPE;
	
	ush_t m_attack = 0.5;
	ush_t m_decay = 0.5;
	ush_t m_sustain = 1.0;
	ush_t m_release = 0.5;

public:
	// TODO: fix this
	EnvelopeCurve(std::vector<EnvPoint>& points);
	/* : m_attack(0.5),
		m_decay(0.5),
		m_sustain(1.0),
		m_release(0.5),
		m_tempo(false),
		m_global(false),
		m_isOn(true)*/
	inline ush_t getAttack() const;
	inline ush_t getDecay() const;
	inline ush_t getSustain() const;
	inline ush_t getRelease() const;

	inline bool isTempo() const;
	inline bool isGlobal() const;
	inline bool isOn() const;
};

class LfoCurve : protected SharedEnvCurve {
private:

	CurveType m_type = CurveType::LFO;
	
	ush_t m_speed;
	ush_t m_lfoTension; // the name is a bit ambiguous with point tension
	ush_t m_skew;
	ush_t m_pulsewidth;

	double m_phase;

	bool m_isBipolar;
	bool m_isFrozen;

public:
	LfoCurve(std::vector<EnvPoint>& points);
	
	ush_t getSpeed() const;
	ush_t getLfoTension() const;
	ush_t getSkew() const;
	ush_t getPulsewidth() const;
	double getPhase() const;
	bool isTempo() const;
	bool isGlobal() const;
	bool isOn() const;

};

