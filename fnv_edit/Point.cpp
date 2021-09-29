#include"Point.h"
#include<iostream>
#include<stdexcept>
#include<string>

Point::Point(ush x, ush y, Mode mode) {
	
	setX(x);
	setY(y);
	setMode(mode);
}


void Point::setX(ush x) {
	//TODO: figure out the limits on the x value
	m_x = x;
}

void Point::setY(ush y) {

	if (y > 1) {
		std::cout << "Y coordinate must be in range [0,1] (inclusively). Input: " << y << std::endl;
	} else { 
		m_y = y; 
	}
}

void Point::setMode(Mode mode) {
	m_mode = mode;
}

void Point::setNum(unsigned int num) {
	m_num = num;
}

void Point::setTension(double tension) {
	m_tension = tension;
}

ush Point::getX() const{
	return m_x;
}

ush Point::getY() const {
	return m_y;
}

Mode Point::getMode() const {
	return m_mode;
}

int Point::getNum() const {
	return m_num;
}

double Point::getTension() const {
	return m_tension;
}
