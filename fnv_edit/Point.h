#pragma once
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
    DOUBLE_CURVE_3 = 0x0c
};

class Point {
private:
    unsigned int m_num;
    unsigned short m_x;
    unsigned short m_y;

    Mode m_mode;
public:
    Point(unsigned short x, unsigned short y, Mode mode);

    void setX(unsigned short x);
    void setY(unsigned short y);
    void setMode(Mode mode);

    double getX() const;
    double getY() const;
    Mode getMode() const;
    int getNum() const;
};