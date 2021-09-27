#include"Point.h"
#include<iostream>
#include<stdexcept>
#include<string>

Point::Point(unsigned short x, unsigned short y, Mode mode) {
	
	setX(x);
	setY(y);
	setMode(mode);
}


void Point::setX(unsigned short x) {
	//TODO: figure out the limits on the x value
	m_x = x;
}

void Point::setY(unsigned short y) {

	if (y > 1 || y < 0) {
		std::cout << "Y coordinate must be in range [0,1] (inclusively). Input: " << y << std::endl;
	} else { 
		m_y = y; 
	}
}

void Point::setMode(Mode mode) {
	m_mode = mode;
}

double Point::getX() const{
	return m_x;
}

double Point::getY() const {
	return m_y;
}

Mode Point::getMode() const {
	return m_mode;
}

int Point::getNum() const {
	return m_num;
}