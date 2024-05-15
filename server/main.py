from aiohttp import web
from src.socket import app

if __name__ == "__main__":
    web.run_app(app, host="localhost", port=10380)
