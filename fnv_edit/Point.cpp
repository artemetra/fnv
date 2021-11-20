#include <iostream>
#include <stdexcept>
#include <string>
#include <stdint.h> // uint32_t

#include "Curve.hpp"
#include "Point.hpp"

Point::Point(const ush_t& x, const ush_t& y, const Mode& mode) : m_xOffset(), m_mode(mode) {
	
	setXOffset(x);
	setY(y);
}

EnvPoint::EnvPoint(const ush_t& x, const ush_t& y, const Mode& mode, const ArpMode& arpMode) {
	;
}

ush_t Point::calculateAbsoluteX() {
	auto curveVectorRef = partOf->vector();

}

void Point::setXOffset(const ush_t& x) {
	if (x > 1) 
		std::cerr << "X coordinate must be in range [0,1] (inclusively). Received: " << x << std::endl;
	else m_xOffset = x;
}

void EnvPoint::setXOffset(const ush_t& x) {
	m_xOffset = x;
}

void Point::setY(const ush_t& y) {
	// TODO: proper errors/error messages
	if (y > 1)
		std::cerr << "Y coordinate must be in range [0,1] (inclusively). Received: " << y << std::endl;
	else m_y = y; 
}

void Point::setMode(const Mode& mode) {
	m_mode = mode;
}

void Point::setNum(const uint32_t& num) {
	// TODO: add restrictions
	m_num = num;
}

void Point::setTension(const double& tension) {
	m_tension = tension;
}

inline ush_t Point::getXOffset() const {
	return m_xOffset;
}

inline ush_t Point::getXAbsolute() const {
	return m_xAbsolute;
}

inline ush_t Point::getY() const {
	return m_y;
}

inline Mode Point::getMode() const {
	return m_mode;
}

inline uint32_t Point::getNum() const {
	return m_num;
}

inline double Point::getTension() const {
	return m_tension;
}
