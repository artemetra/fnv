#pragma once
#include <stdint.h>
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



typedef unsigned short ush_t;
class Point {
protected:
    uint32_t m_num;
    ush_t m_x;
    ush_t m_y;

    Mode m_mode;
    double m_tension;

public:
    Point(ush_t x, ush_t y, Mode mode);

    void setX(ush_t x);
    void setY(ush_t y);
    void setMode(Mode mode);
    void setNum(uint32_t num);
    void setTension(double tension);

    ush_t getX() const;
    ush_t getY() const;
    Mode getMode() const;
    uint32_t getNum() const;
    double getTension() const;

};

enum class ArpMode {
    NONE = 0x00,
    PREVIOUS = 0x01,
    SAME = 0x02,
    NEXT = 0x03
};

class EnvPoint : protected Point {
private:
    ArpMode m_arpMode;
    
public:
    // TODO: add implementations for these
    EnvPoint();

    void setArpMode(ArpMode arpMode);
    
    ArpMode getArpMode() const;
    
};

