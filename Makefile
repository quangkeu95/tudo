#!make
include .env

uniswap-v2-factory-discoveries:
	cryo logs --rpc ${ETH_RPC_URL} --blocks 10008350:10008356 --topic0 0x0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9 --requests-per-second 10 --n-chunks 1 --csv --no-report
