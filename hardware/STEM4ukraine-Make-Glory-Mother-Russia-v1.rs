ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=A1NNaW8tmMz8NLiExHsAAAAr;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAs; loclib_name=led5;
						li:objects {
						}
						ha:attrib {
							device=led5
							footprint=LED5
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.2 {
						uuid=A1NNaW8tmMz8NLiExHsAAAD6; loclib_name=3mmLED_backplane;
						li:objects {
						}
						ha:attrib {
							footprint=3mmLEDbackplane.lht
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=A1NNaW8tmMz8NLiExHsAAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.sheet-decor-fill { shape=round; size=125; color=#bbbbbb; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-decor-fill { shape=round; size=125; color=#99ff99; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.2 {
				uuid=A1NNaW8tmMz8NLiExHsAAAAW; src_uuid=/iiShtebwvwwWnNJ68YAAAAJ;
				x=16000; y=20000;
				li:objects {
					ha:text.1 { x1=0; y1=-8000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=28000; }
							ha:line { x1=0; y1=28000; x2=16000; y2=28000; }
							ha:line { x1=16000; y1=28000; x2=16000; y2=0; }
							ha:line { x1=16000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAX; src_uuid=/iiShtebwvwwWnNJ68YAAAAB;
						x=16000; y=24000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB0
							pinnum=5
							role=terminal
						}
					}
					ha:group.4 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAY; src_uuid=/iiShtebwvwwWnNJ68YAAAAC;
						x=16000; y=20000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB1
							pinnum=6
							role=terminal
						}
					}
					ha:group.5 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAZ; src_uuid=/iiShtebwvwwWnNJ68YAAAAD;
						x=16000; y=16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB2
							pinnum=7
							role=terminal
						}
					}
					ha:group.6 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAa; src_uuid=/iiShtebwvwwWnNJ68YAAAAE;
						x=16000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB3
							pinnum=2
							role=terminal
						}
					}
					ha:group.7 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAb; src_uuid=/iiShtebwvwwWnNJ68YAAAAF;
						x=16000; y=8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB4
							pinnum=3
							role=terminal
						}
					}
					ha:group.8 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAc; src_uuid=/iiShtebwvwwWnNJ68YAAAAG;
						x=16000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB5
							pinnum=1
							role=terminal
						}
					}
					ha:group.9 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAd; src_uuid=/iiShtebwvwwWnNJ68YAAAAH;
						x=8000; y=28000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VCC
							pinnum=8
							role=terminal
						}
					}
					ha:group.10 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAe; src_uuid=/iiShtebwvwwWnNJ68YAAAAI;
						x=8000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=4
							role=terminal
						}
					}
				}
				ha:attrib {
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					footprint=so(8)
					name=U1
					role=symbol
				}
			}
			ha:group.4 {
				uuid=A1NNaW8tmMz8NLiExHsAAAAo; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=128000; y=8000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAp; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=A1NNaW8tmMz8NLiExHsAAAAq; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.9 { x1=2000; y1=5000; rot=90.000000; dyntext=1; stroke=sym-secondary; text=%../a.devmap%; floater=1; }
					ha:text.10 { x1=-2000; y1=5000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_backplane
					name=D1
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.5 {
				uuid=A1NNaW8tmMz8NLiExHsAAAAz; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=104000; y=40000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAA0; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=A1NNaW8tmMz8NLiExHsAAAA1; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=4000; y1=2000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=4000; y1=5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=1206
					name=R1
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=320R
				}
			}
			ha:group.7 {
				uuid=A1NNaW8tmMz8NLiExHsAAABH; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAH;
				x=152000; y=28000;
				li:objects {
					ha:text.1 { x1=-2000; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=20000; }
							ha:line { x1=0; y1=20000; x2=16000; y2=20000; }
							ha:line { x1=16000; y1=20000; x2=16000; y2=0; }
							ha:line { x1=16000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=A1NNaW8tmMz8NLiExHsAAABI; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAB;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=MISO
							pinnum=1
							role=terminal
						}
					}
					ha:group.4 {
						uuid=A1NNaW8tmMz8NLiExHsAAABJ; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAC;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SCK
							pinnum=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=A1NNaW8tmMz8NLiExHsAAABK; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAD;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=MOSI
							pinnum=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=A1NNaW8tmMz8NLiExHsAAABL; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAE;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=RST
							pinnum=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=A1NNaW8tmMz8NLiExHsAAABM; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAF;
						x=8000; y=20000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Vcc
							pinnum=2
							role=terminal
						}
					}
					ha:group.8 {
						uuid=A1NNaW8tmMz8NLiExHsAAABN; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAG;
						x=8000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=6
							role=terminal
						}
					}
					ha:text.9 { x1=-2000; y1=-8000; dyntext=1; stroke=sym-secondary; text=%../A.device%; floater=1; }
				}
				ha:attrib {
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					device=ISP6pin
					footprint={connector(2,3,sequence=pivot,silkmark=external)}
					name=ISP
					role=symbol
				}
			}
			ha:group.9 {
				uuid=A1NNaW8tmMz8NLiExHsAAABX;
				x=-4000; y=0;
				li:objects {
					ha:line.1 { x1=48000; y1=44000; x2=40000; y2=44000; stroke=wire; }
					ha:text.2 { x1=44000; y1=44000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=MOSI
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.11 {
				uuid=A1NNaW8tmMz8NLiExHsAAABY;
				x=-4000; y=0;
				li:objects {
					ha:line.1 { x1=48000; y1=40000; x2=40000; y2=40000; stroke=wire; }
					ha:text.3 { x1=44000; y1=40000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=MISO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.13 {
				uuid=A1NNaW8tmMz8NLiExHsAAABZ;
				x=-4000; y=0;
				li:objects {
					ha:line.1 { x1=48000; y1=36000; x2=40000; y2=36000; stroke=wire; }
					ha:text.2 { x1=44000; y1=36000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SCK
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.15 {
				uuid=A1NNaW8tmMz8NLiExHsAAABa;
				li:objects {
					ha:line.1 { x1=36000; y1=24000; x2=48000; y2=24000; stroke=wire; }
					ha:text.2 { x1=40000; y1=24000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=RESET
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.17 {
				uuid=A1NNaW8tmMz8NLiExHsAAABb;
				x=12000; y=0;
				li:objects {
					ha:line.1 { x1=136000; y1=44000; x2=128000; y2=44000; stroke=wire; }
					ha:text.2 { x1=128000; y1=44000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
					ha:text.3 { x1=128000; y1=44000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=MISO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.19 {
				uuid=A1NNaW8tmMz8NLiExHsAAABc;
				x=12000; y=0;
				li:objects {
					ha:line.1 { x1=128000; y1=40000; x2=136000; y2=40000; stroke=wire; }
					ha:text.2 { x1=128000; y1=40000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SCK
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.21 {
				uuid=A1NNaW8tmMz8NLiExHsAAABd;
				x=12000; y=0;
				li:objects {
					ha:line.1 { x1=136000; y1=36000; x2=128000; y2=36000; stroke=wire; }
					ha:text.2 { x1=128000; y1=36000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=MOSI
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.23 {
				uuid=A1NNaW8tmMz8NLiExHsAAABe;
				x=12000; y=0;
				li:objects {
					ha:line.1 { x1=128000; y1=32000; x2=136000; y2=32000; stroke=wire; }
					ha:text.2 { x1=128000; y1=32000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=RESET
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.25 {
				uuid=A1NNaW8tmMz8NLiExHsAAABf;
				li:objects {
					ha:line.1 { x1=36000; y1=32000; x2=48000; y2=32000; stroke=wire; }
					ha:text.2 { x1=40000; y1=32000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=LED
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.38 {
				uuid=A1NNaW8tmMz8NLiExHsAAABp; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=128000; y=4000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAABq; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.39 {
				uuid=A1NNaW8tmMz8NLiExHsAAABr; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=24000; y=5000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAABs; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.40 {
				uuid=A1NNaW8tmMz8NLiExHsAAABt;
				x=-4000; y=0;
				li:objects {
					ha:line.1 { x1=28000; y1=5000; x2=28000; y2=16000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.46 {
				uuid=A1NNaW8tmMz8NLiExHsAAABv;
				x=24000; y=0;
				li:objects {
					ha:line.1 { x1=104000; y1=4000; x2=104000; y2=8000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.49 {
				uuid=A1NNaW8tmMz8NLiExHsAAABz; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=160000; y=4000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAB0; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.50 {
				uuid=A1NNaW8tmMz8NLiExHsAAAB1; src_uuid=A1NNaW8tmMz8NLiExHsAAABv;
				x=56000; y=0;
				li:objects {
					ha:line.1 { x1=104000; y1=4000; x2=104000; y2=24000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.53 {
				uuid=A1NNaW8tmMz8NLiExHsAAAB6; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=24000; y=56000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAB7; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=5V
					role=symbol
				}
			}
			ha:group.54 {
				uuid=A1NNaW8tmMz8NLiExHsAAAB8;
				x=-4000; y=0;
				li:objects {
					ha:line.1 { x1=28000; y1=56000; x2=28000; y2=52000; stroke=wire; }
					ha:line.2 { x1=28000; y1=52000; x2=48000; y2=52000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.57 {
				uuid=A1NNaW8tmMz8NLiExHsAAACA; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=160000; y=56000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAACB; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=5V
					role=symbol
				}
			}
			ha:group.58 {
				uuid=A1NNaW8tmMz8NLiExHsAAACC; src_uuid=A1NNaW8tmMz8NLiExHsAAAB8;
				x=132000; y=0;
				li:objects {
					ha:line.1 { x1=28000; y1=56000; x2=28000; y2=52000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.73 {
				uuid=A1NNaW8tmMz8NLiExHsAAADT; src_uuid=A1NNaW8tmMz8NLiExHsAAADM;
				x=192000; y=44000; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=A1NNaW8tmMz8NLiExHsAAADU; src_uuid=A1NNaW8tmMz8NLiExHsAAADN;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=A1NNaW8tmMz8NLiExHsAAADV; src_uuid=A1NNaW8tmMz8NLiExHsAAADO;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=A1NNaW8tmMz8NLiExHsAAADW; src_uuid=A1NNaW8tmMz8NLiExHsAAADP;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=A1NNaW8tmMz8NLiExHsAAADX; src_uuid=A1NNaW8tmMz8NLiExHsAAADQ;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=A1NNaW8tmMz8NLiExHsAAADY; src_uuid=A1NNaW8tmMz8NLiExHsAAADR;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=A1NNaW8tmMz8NLiExHsAAADZ; src_uuid=A1NNaW8tmMz8NLiExHsAAADS;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=22000; }
							ha:line { x1=0; y1=22000; x2=4000; y2=22000; }
							ha:line { x1=4000; y1=22000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=USB_B_180_degree_PTH.rf
					name=USB
					role=symbol
					spice/omit=yes
				}
			}
			ha:group.74 {
				uuid=A1NNaW8tmMz8NLiExHsAAADd; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=180000; y=56000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAADe; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=5V
					role=symbol
				}
			}
			ha:group.75 {
				uuid=A1NNaW8tmMz8NLiExHsAAADf; src_uuid=A1NNaW8tmMz8NLiExHsAAAB8;
				x=152000; y=0;
				li:objects {
					ha:line.1 { x1=28000; y1=56000; x2=28000; y2=44000; stroke=wire; }
					ha:line.2 { x1=28000; y1=44000; x2=36000; y2=44000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.78 {
				uuid=A1NNaW8tmMz8NLiExHsAAADg;
				x=20000; y=0;
				li:objects {
					ha:line.1 { x1=168000; y1=32000; x2=160000; y2=32000; stroke=wire; }
					ha:line.2 { x1=160000; y1=32000; x2=160000; y2=4000; stroke=wire; }
					ha:line.3 { x1=160000; y1=28000; x2=168000; y2=28000; stroke=wire; }
					ha:line.4 { x1=160000; y1=28000; x2=160000; y2=28000; stroke=junction; }
					ha:line.5 { x1=168000; y1=24000; x2=160000; y2=24000; stroke=wire; }
					ha:line.6 { x1=160000; y1=24000; x2=160000; y2=24000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.82 {
				uuid=A1NNaW8tmMz8NLiExHsAAADj; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=180000; y=4000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAADk; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.84 {
				uuid=A1NNaW8tmMz8NLiExHsAAADr; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=208000; y=44000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAADs; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=A1NNaW8tmMz8NLiExHsAAADt; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=capacitor; prio=31050; }
					footprint=1206
					name=C1
					role=symbol
					ha:spice/prefix = { value=C; prio=31050; }
					value=100n
				}
			}
			ha:group.85 {
				uuid=A1NNaW8tmMz8NLiExHsAAADx; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=208000; y=56000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAADy; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=5V
					role=symbol
				}
			}
			ha:group.86 {
				uuid=A1NNaW8tmMz8NLiExHsAAADz; src_uuid=A1NNaW8tmMz8NLiExHsAAADw;
				x=212000; y=44000;
				li:objects {
					ha:line.1 { x1=-4000; y1=12000; x2=-4000; y2=0; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.89 {
				uuid=A1NNaW8tmMz8NLiExHsAAAD3; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=208000; y=4000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAD4; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.90 {
				uuid=A1NNaW8tmMz8NLiExHsAAAD5; src_uuid=A1NNaW8tmMz8NLiExHsAAABv;
				x=104000; y=0;
				li:objects {
					ha:line.1 { x1=104000; y1=4000; x2=104000; y2=24000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.93 {
				uuid=A1NNaW8tmMz8NLiExHsAAAEB; src_uuid=3s0ePx27Ce5+YP4+xH0AAADp;
				x=104000; y=20000; rot=270.000000;
				li:objects {
					ha:arc.1 { cx=6000; cy=9000; r=6000; sang=0.000000; dang=-180.000000; stroke=sym-decor; }
					ha:group.2 {
						uuid=A1NNaW8tmMz8NLiExHsAAAEC; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB4;
						x=12000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.3 {
						uuid=A1NNaW8tmMz8NLiExHsAAAED; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB5;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.4 { x1=0; y1=9000; x2=12000; y2=9000; stroke=sym-decor; }
					ha:line.5 { x1=4000; y1=0; x2=4000; y2=3343; stroke=sym-decor; }
					ha:line.6 { x1=8000; y1=0; x2=8000; y2=3343; stroke=sym-decor; }
					ha:text.7 { x1=0; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					footprint=piezo_5mm_to_7.62mm.rf
					name=PIEZO
					role=symbol
				}
			}
			ha:group.94 {
				uuid=A1NNaW8tmMz8NLiExHsAAAEH; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=104000; y=4000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAEI; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.95 {
				uuid=A1NNaW8tmMz8NLiExHsAAAEJ; src_uuid=A1NNaW8tmMz8NLiExHsAAABv;
				x=-20000; y=0;
				li:objects {
					ha:line.1 { x1=124000; y1=4000; x2=124000; y2=8000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.111 {
				uuid=A1NNaW8tmMz8NLiExHsAAAE/; src_uuid=A1NNaW8tmMz8NLiExHsAAAE8;
				x=52000; y=24000;
				li:objects {
					ha:text.1 { x1=-4000; y1=-5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=A1NNaW8tmMz8NLiExHsAAAFA; src_uuid=A1NNaW8tmMz8NLiExHsAAAE9;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:line.3 { x1=0; y1=0; x2=1200; y2=0; stroke=sym-decor; }
					ha:group.4 {
						uuid=A1NNaW8tmMz8NLiExHsAAAFB; src_uuid=A1NNaW8tmMz8NLiExHsAAAE+;
						x=8000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							footprint=switch_SMD_tactile_3mmx6mm.rf
							name=2
							role=terminal
						}
					}
					ha:line.5 { x1=6800; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:arc.6 { cx=6400; cy=0; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.7 { x1=1200; y1=0; x2=5600; y2=3200; stroke=sym-decor; }
					ha:line.8 { x1=4000; y1=2000; x2=4000; y2=-2000; stroke=sym-decor; }
				}
				ha:attrib {
					footprint=switch_SMD_tactile_3mmx6mm.rf
					name=SW1
					role=symbol
					spice/omit=yes
				}
			}
			ha:group.130 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFS; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=44000; y=52000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAFT; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=A1NNaW8tmMz8NLiExHsAAAFU; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=4000; y1=2000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=4000; y1=5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=1206
					name=R2
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=10k
				}
			}
			ha:connection.131 {
				li:conn {
					/2/9/1
					/2/2/3/1
				}
			}
			ha:connection.132 {
				li:conn {
					/2/11/1
					/2/2/4/1
				}
			}
			ha:connection.133 {
				li:conn {
					/2/13/1
					/2/2/5/1
				}
			}
			ha:connection.134 {
				li:conn {
					/2/2/8/1
					/2/15/1
				}
			}
			ha:connection.135 {
				li:conn {
					/2/40/1
					/2/2/10/1
				}
			}
			ha:connection.136 {
				li:conn {
					/2/40/1
					/2/39/1/1
				}
			}
			ha:connection.137 {
				li:conn {
					/2/54/1
					/2/2/9/1
					/2/54/2
				}
			}
			ha:connection.138 {
				li:conn {
					/2/54/1
					/2/53/1/1
				}
			}
			ha:connection.139 {
				li:conn {
					/2/2/6/1
					/2/25/1
				}
			}
			ha:group.145 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=72000; y=24000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAFa; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=A1NNaW8tmMz8NLiExHsAAAFb; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=4000; y1=2000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=4000; y1=5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=1206
					name=R3
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=47k
				}
			}
			ha:connection.153 {
				li:conn {
					/2/54/2
					/2/130/2/1
				}
			}
			ha:connection.155 {
				li:conn {
					/2/111/2/1
					/2/15/1
				}
			}
			ha:group.157 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFe;
				li:objects {
					ha:text.1 { x1=40000; y1=28000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
					ha:line.2 { x1=36000; y1=28000; x2=48000; y2=28000; stroke=wire; }
				}
				ha:attrib {
					name=PIEZO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.158 {
				li:conn {
					/2/157/2
					/2/2/7/1
				}
			}
			ha:group.184 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFf;
				x=4000; y=0;
				li:objects {
					ha:line.2 { x1=88000; y1=24000; x2=92000; y2=24000; stroke=wire; }
					ha:line.3 { x1=92000; y1=4000; x2=92000; y2=24000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.187 {
				li:conn {
					/2/17/1
					/2/7/3/1
				}
			}
			ha:connection.188 {
				li:conn {
					/2/19/1
					/2/7/4/1
				}
			}
			ha:connection.189 {
				li:conn {
					/2/21/1
					/2/7/5/1
				}
			}
			ha:connection.190 {
				li:conn {
					/2/23/1
					/2/7/6/1
				}
			}
			ha:connection.194 {
				li:conn {
					/2/50/1
					/2/7/8/1
				}
			}
			ha:connection.195 {
				li:conn {
					/2/50/1
					/2/49/1/1
				}
			}
			ha:connection.196 {
				li:conn {
					/2/58/1
					/2/57/1/1
				}
			}
			ha:connection.197 {
				li:conn {
					/2/58/1
					/2/7/7/1
				}
			}
			ha:connection.198 {
				li:conn {
					/2/75/1
					/2/74/1/1
				}
			}
			ha:connection.199 {
				li:conn {
					/2/75/2
					/2/73/2/1
				}
			}
			ha:connection.200 {
				li:conn {
					/2/78/1
					/2/73/5/1
				}
			}
			ha:connection.201 {
				li:conn {
					/2/78/3
					/2/73/6/1
				}
			}
			ha:connection.202 {
				li:conn {
					/2/78/5
					/2/73/7/1
				}
			}
			ha:connection.203 {
				li:conn {
					/2/82/1/1
					/2/78/2
				}
			}
			ha:connection.204 {
				li:conn {
					/2/86/1
					/2/85/1/1
				}
			}
			ha:connection.205 {
				li:conn {
					/2/86/1
					/2/84/2/1
				}
			}
			ha:connection.206 {
				li:conn {
					/2/90/1
					/2/84/1/1
				}
			}
			ha:connection.207 {
				li:conn {
					/2/90/1
					/2/89/1/1
				}
			}
			ha:group.211 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFg;
				x=4000; y=0;
				li:objects {
					ha:line.1 { x1=92000; y1=40000; x2=100000; y2=40000; stroke=wire; }
					ha:text.2 { x1=92000; y1=40000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=LED
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.217 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFi;
				x=4000; y=0;
				li:objects {
					ha:line.1 { x1=124000; y1=40000; x2=120000; y2=40000; stroke=wire; }
					ha:line.2 { x1=124000; y1=24000; x2=124000; y2=40000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.218 {
				li:conn {
					/2/217/1
					/2/5/1/1
				}
			}
			ha:group.219 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFj;
				x=4000; y=0;
				li:objects {
					ha:line.2 { x1=92000; y1=32000; x2=100000; y2=32000; stroke=wire; }
					ha:text.3 { x1=92000; y1=32000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
					ha:line.4 { x1=100000; y1=20000; x2=100000; y2=32000; stroke=wire; }
				}
				ha:attrib {
					name=PIEZO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.222 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFm; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=96000; y=4000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAFn; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.224 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFo;
				li:objects {
					ha:line.1 { x1=64000; y1=24000; x2=72000; y2=24000; stroke=wire; }
					ha:line.2 { x1=68000; y1=24000; x2=68000; y2=52000; stroke=wire; }
					ha:line.3 { x1=68000; y1=52000; x2=64000; y2=52000; stroke=wire; }
					ha:line.4 { x1=68000; y1=24000; x2=68000; y2=24000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.225 {
				li:conn {
					/2/224/1
					/2/111/4/1
				}
			}
			ha:connection.227 {
				li:conn {
					/2/224/3
					/2/130/1/1
				}
			}
			ha:group.229 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFw; src_uuid=A1NNaW8tmMz8NLiExHsAAADw;
				x=228000; y=44000;
				li:objects {
					ha:line.1 { x1=-4000; y1=12000; x2=-4000; y2=-8000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.230 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFx; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=224000; y=4000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAFy; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.231 {
				uuid=A1NNaW8tmMz8NLiExHsAAAFz; src_uuid=A1NNaW8tmMz8NLiExHsAAABv;
				x=120000; y=0;
				li:objects {
					ha:line.1 { x1=104000; y1=4000; x2=104000; y2=32000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.232 {
				li:conn {
					/2/231/1
					/2/230/1/1
				}
			}
			ha:group.233 {
				uuid=A1NNaW8tmMz8NLiExHsAAAF0; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=224000; y=56000;
				li:objects {
					ha:group.1 {
						uuid=A1NNaW8tmMz8NLiExHsAAAF1; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=5V
					role=symbol
				}
			}
			ha:connection.234 {
				li:conn {
					/2/233/1/1
					/2/229/1
				}
			}
			ha:group.235 {
				uuid=A1NNaW8tmMz8NLiExHsAAAGY; src_uuid=A1NNaW8tmMz8NLiExHsAAAGV;
				x=228000; y=36000; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=A1NNaW8tmMz8NLiExHsAAAGZ; src_uuid=A1NNaW8tmMz8NLiExHsAAAGW;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=A1NNaW8tmMz8NLiExHsAAAGa; src_uuid=A1NNaW8tmMz8NLiExHsAAAGX;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=EXT_5V
					role=symbol
					spice/omit=yes
				}
			}
			ha:connection.238 {
				li:conn {
					/2/235/2/1
					/2/229/1
				}
			}
			ha:connection.239 {
				li:conn {
					/2/235/3/1
					/2/231/1
				}
			}
			ha:connection.240 {
				li:conn {
					/2/46/1
					/2/38/1/1
				}
			}
			ha:connection.241 {
				li:conn {
					/2/46/1
					/2/4/1/1
				}
			}
			ha:connection.242 {
				li:conn {
					/2/95/1
					/2/93/2/1
				}
			}
			ha:connection.243 {
				li:conn {
					/2/95/1
					/2/94/1/1
				}
			}
			ha:connection.244 {
				li:conn {
					/2/211/1
					/2/5/2/1
				}
			}
			ha:connection.245 {
				li:conn {
					/2/217/2
					/2/4/2/1
				}
			}
			ha:connection.246 {
				li:conn {
					/2/219/4
					/2/93/3/1
				}
			}
			ha:connection.247 {
				li:conn {
					/2/222/1/1
					/2/184/3
				}
			}
			ha:connection.248 {
				li:conn {
					/2/145/1/1
					/2/184/2
				}
			}
			ha:connection.249 {
				li:conn {
					/2/224/1
					/2/145/2/1
				}
			}
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 0
     grid = 1.0240 mm
    }
   }
  }
}
