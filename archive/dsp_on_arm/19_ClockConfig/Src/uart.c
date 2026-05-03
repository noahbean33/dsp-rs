#include "uart.h"


#define GPIOAEN			(1U<<0)
#define UART2EN			(1U<<17)
#define CR1_TE			(1U<<3)
#define CR1_UE			(1U<<13)

#define SR_TXE			(1U<<7)

/*
#define SYS_FREQ		16000000
#define APB1_CLK		SYS_FREQ
*/


#define SYS_FREQ		100000000
#define APB1_CLK		(SYS_FREQ/2)

#define UART_BAUDRATE	115200

static void uart2_set_baudrate(uint32_t periph_clk, uint32_t baudrate);

void uart2_write(int ch);

int __io_putchar(int ch)
{
	uart2_write(ch);

	return ch;
}

void uart2_tx_init(void)
{
	/*****Configure UART GPIO Pin******/
	/*Enable clock access to GPIOA*/
	RCC->AHB1ENR |= GPIOAEN;

	/*Set PA2 mode to alternate function mode */
	GPIOA->MODER &=~(1U<<4);
	GPIOA->MODER |=(1U<<5);

	/*Set PA2 alternate function type to UART_TX(AF07)*/
	GPIOA->AFR[0] |=(1U<<8);
	GPIOA->AFR[0] |=(1U<<9);
	GPIOA->AFR[0] |=(1U<<10);
	GPIOA->AFR[0] &=~(1U<<11);

	/*****Configure UART  ******/
	/*Enable clock access to UART2*/
	RCC->APB1ENR |=UART2EN;

	/*Configure baudrate*/
	uart2_set_baudrate(APB1_CLK,UART_BAUDRATE);

	/*Configure the transfer direction*/
	USART2->CR1 = CR1_TE;

	/*Enable UART module*/
	USART2->CR1 |= CR1_UE;

}

void uart2_write(int ch)
{
	/*Make sure transmit data register is empty*/
	   while(!(USART2->SR & SR_TXE)){}

	/*Write to the transmit data register*/
	   USART2->DR = (ch &0xFF);
}

static uint16_t compute_uart_bd( uint32_t periph_clk, uint32_t baudrate)
{
	return ((periph_clk + (baudrate/2U))/baudrate);
}

static void uart2_set_baudrate(uint32_t periph_clk, uint32_t baudrate)
{
	USART2->BRR = compute_uart_bd(periph_clk,baudrate);
}

