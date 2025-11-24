# Autopilot - Debug

# ================================================================================================ #

import pygame as pg

# ================================================================================================ #

pg.init()
font = pg.font.Font(None, 30)

def debug(info, y = 10, x = 10):
	display_surface	= pg.display.get_surface()
	debug_surface	= font.render(str(info), True, 'White')
	debug_rect		= debug_surface.get_rect(topleft = (x, y))
	
	pg.draw.rect(display_surface, 'Black', debug_rect)
	display_surface.blit(debug_surface, debug_rect)