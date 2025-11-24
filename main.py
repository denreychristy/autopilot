# AutoPilot - Main

# ================================================================================================ #
# Imports

import sys
import pygame as pg

from rust import Currency # type: ignore
from rust import NonPlayerCharacter as NPC # type: ignore

from settings import Settings
from debug import debug

# ================================================================================================ #

class AutoPilot:
	def __init__(self):
		self.settings = Settings()

		pg.init()
		self.screen = pg.display.set_mode((self.settings.WIDTH, self.settings.HEIGHT))
		pg.display.set_caption('Autopilot')
		self.clock = pg.time.Clock()
	
	def run(self):
		while True:
			for event in pg.event.get():
				if event.type == pg.QUIT:
					pg.quit()
					sys.exit()
			
			self.screen.fill('black')
			debug('Debug')
			pg.display.update()
			self.clock.tick(self.settings.FPS)

# ================================================================================================ #

if __name__ == '__main__':
	autopilot = AutoPilot()
	autopilot.run()