# AutoPilot - Main

# ================================================================================================ #
# Imports

import sys
import pygame as pg

from time import time
from random import randint

from rust import Currency # type: ignore
from rust import NonPlayerCharacter as NPC # type: ignore
from rust import Tile # type: ignore
from rust import World # type: ignore

from settings import Settings
from debug import debug

# ================================================================================================ #

test_tile = pg.image.load('images/tiles/test_tile_3.png')

# ================================================================================================ #

class AutoPilot:
	def __init__(self):
		self.settings = Settings()

		pg.init()
		self.screen = pg.display.set_mode((self.settings.WIDTH, self.settings.HEIGHT))
		pg.display.set_caption('Autopilot')
		self.clock = pg.time.Clock()

		self.world = World()
		for _ in range(100):
			x = randint(-1000, 1000)
			y = randint(-1000, 1000)
			print(f"Fetching map tile at coordinates ({x}, {y}).")
			self.world.get(x, y)
	
	def run(self):
		while True:
			for event in pg.event.get():
				if event.type == pg.QUIT:
					pg.quit()
					sys.exit()
			
			self.display()
			self.clock.tick(self.settings.FPS)
	
	def display(self):
		# Window Background
		self.screen.fill('black')

		# Ground Tiles
		for y in range(50):
			for x in range(50):
				self.screen.blit(test_tile, (32 * x, 16 * y))
		
		if self.settings.flag_debug: debug('Debug')
		pg.display.update()

# ================================================================================================ #

if __name__ == '__main__':
	autopilot = AutoPilot()
	autopilot.run()