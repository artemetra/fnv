#include<iostream>
#include<stdexcept>
#include<string>
#include<stdint.h>

#include"Point.hpp"

Point::Point(ush_t x, ush_t y, Mode mode) {
	
	setX(x);
	setY(y);
	setMode(mode);
}


void Point::setX(ush_t x) {
	//TODO: figure out the limits on the x value
	m_x = x;
}

void Point::setY(ush_t y) {

	if (y > 1) {
		std::cout << "Y coordinate must be in range [0,1] (inclusively). Input: " << y << std::endl;
	} else { 
		m_y = y; 
	}
}

void Point::setMode(Mode mode) {
	m_mode = mode;
}

void Point::setNum(uint32_t num) {
	m_num = num;
}

void Point::setTension(double tension) {
	m_tension = tension;
}

ush_t Point::getX() const{
	return m_x;
}

ush_t Point::getY() const {
	return m_y;
}

Mode Point::getMode() const {
	return m_mode;
}

double Point::getTension() const {
	return m_tension;
}
