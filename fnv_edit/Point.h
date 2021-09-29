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



typedef unsigned short ush;
class Point {
protected:
    unsigned int m_num;
    ush m_x;
    ush m_y;

    Mode m_mode;
    double m_tension;

public:
    Point(ush x, ush y, Mode mode);

    void setX(ush x);
    void setY(ush y);
    void setMode(Mode mode);
    void setNum(unsigned int num);
    void setTension(double tension);

    ush getX() const;
    ush getY() const;
    Mode getMode() const;
    int getNum() const;
    double getTension() const;

};

enum class ArpMode {
    // TODO: reverse engineer the bytes corresponding to
    // arpeggiator breaks
    NONE,
    PREVIOUS,
    SAME,
    NEXT
};

struct AdsrMode
{
    bool decay;
    bool loopStart;
    bool sustain;
    bool* loopEnd = &sustain;
    // in fl, it's written as "Sustain / Loop End" so i
    // might use either of them
};


class EnvPoint : protected Point {
private:
    ArpMode m_arpMode;
    AdsrMode m_adsrMode;
public:
    // TODO: add implementations for these
    EnvPoint();

    void setArpMode(ArpMode arpMode);
    void setAdsrMode(AdsrMode adsrMode);

    ArpMode getArpMode() const;
    AdsrMode getAdsrMode() const;
};

