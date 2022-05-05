#pragma once

#include <stdint.h> // uint32_t
#include <memory> // shared_ptr

typedef unsigned short ush_t;

enum class Mode
{
	/*
		| Name           | Short* | Byte   |
		| --------       | -----  | :--:   |
		| Hold           | HD     | `02`   |
		| Smooth         | SM     | `0A`   |
		| Single Curve   | SC     | `00`   |
		| Single Curve 2 | SC2    | `07`   |
		| Single Curve 3 | SC3    | `0B`   |
		| Double Curve   | DC     | `01`   |
		| Double Curve 2 | DC2    | `01`   |
		| Double Curve 3 | DC3    | `08`   |
		| Half Sine      | HS     | `09`   |
		| Stairs         | ST     | `03`   |
		| Smooth Stairs  | SST    | `04`   |
		| Pulse          | PU     | `05`   |
		| Wave           | WV     | `06`   |

		| Byte   | Name           | Short  |
		| :--:   | --------       | -----  |
		| `00`   | Single Curve   | SC     |
		| `01`   | Double Curve   | DC     |
		| `02`   | Hold           | HD     |
		| `03`   | Stairs         | ST     |
		| `04`   | Smooth Stairs  | SST    |
		| `05`   | Pulse          | PU     |
		| `06`   | Wave           | WV     |
		| `07`   | Single Curve 2 | SC2    |
		| `08`   | Double Curve 2 | DC2    |
		| `09`   | Half Sine      | HS     |
		| `0A`   | Smooth         | SM     |
		| `0B`   | Single Curve 3 | SC3    |
		| `0C`   | Double Curve 3 | DC3    |
	*/
	SINGLE_CURVE = 0x00,
	DOUBLE_CURVE = 0x01,
	HOLD = 0x02,
	STAIRS = 0x03,
	SMOOTH_STAIRS = 0x04,
	PULSE = 0x05,
	WAVE = 0x06,
	SINGLE_CURVE_2 = 0x07,
	DOUBLE_CURVE_2 = 0x08,
	HALF_SINE = 0x09,
	SMOOTH = 0x0A,
	SINGLE_CURVE_3 = 0x0B,
	DOUBLE_CURVE_3 = 0x0C
};

// used in Curve* isPartOf (line 69) because
// #include "Curve.hpp" leads to circular inclusion
class Curve; 

enum class ArpMode {
	NONE = 0x00,
	PREV = 0x01,
	SAME = 0x02,
	NEXT = 0x03
};

struct PointDefArgs {
	// if the x value passed to the constructor is
	// an offset value or an absolute one
	Mode mode = Mode::SINGLE_CURVE;
	double tension = 0;
	ArpMode arpMode = ArpMode::NONE;
};

class Point {
protected:
	uint32_t m_num;
	ush_t m_xOffset;
	ush_t m_xAbsolute;
	ush_t m_y;

	Mode m_mode;
	double m_tension;

	//void m_setDefaultParams(const PointDefArgs& defParams);

public:
	const std::shared_ptr<Curve> partOf;

	Point(const std::shared_ptr<Curve> _partOf, 
		  const ush_t& x, const ush_t& y, 
		  const PointDefArgs& defParams);
	Point() = default;
	virtual void setXOffset(const ush_t& x);
	void setY(const ush_t& y);
	void setMode(const Mode& mode);
	void setNum(const uint32_t& num);
	void setTension(const double& tension);

	virtual double calculateAbsoluteX();
	inline ush_t getXOffset() const;
	inline ush_t getXAbsolute() const;
	inline ush_t getY() const;
	inline Mode getMode() const;
	inline uint32_t getNum() const;
	inline double getTension() const;

};


class EnvPoint : protected Point {
private:
	ArpMode m_arpMode = ArpMode::NONE;
	
public:
	// TODO: add implementations for these
	EnvPoint(const ush_t& x, const ush_t& y, const Mode& mode, const ArpMode& arpMode);

	double calculateAbsoluteX();
	void setArpMode(ArpMode arpMode);
	void setXOffset(const ush_t& x);
	inline ArpMode getArpMode() const;
	
};

