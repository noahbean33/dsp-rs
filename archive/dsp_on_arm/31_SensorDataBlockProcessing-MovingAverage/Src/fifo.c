#include "fifo.h"


rx_dataType	RX_FIFO[RXFIFOSIZE];

volatile rx_dataType * rx_put_pt;
volatile rx_dataType * rx_get_pt;


/*Initialize the fifo*/

void rx_fifo_init(void)
{
	/*Reset fifo*/
	rx_put_pt =  rx_get_pt =  &RX_FIFO[0];
}

/*Put data into rx fifo*/
uint8_t rx_fifo_put(rx_dataType data)
{
	rx_dataType volatile *rx_next_put_pt;

	rx_next_put_pt =  rx_put_pt + 1;

	/*Check if at the end*/
	if( rx_next_put_pt == &RX_FIFO[RXFIFOSIZE])
	{
		/*Wrap around*/
		rx_next_put_pt =  &RX_FIFO[0];
	}

	if(rx_next_put_pt == rx_get_pt)
	{
		return (RXFIFOFAIL);
	}
	else
	{
		/*Put data into fifo*/
		*(rx_put_pt) =  data;

		 rx_put_pt  =  rx_next_put_pt;

		 return (RXFIFOSUCCESS);
	}
}


/*Get data from fifo*/
uint8_t rx_fifo_get(rx_dataType * datapt)
{
	/*Check if fifo is empty*/
	if(rx_put_pt == rx_get_pt)
	{
		/*Fifo empty*/
		return (RXFIFOFAIL);
	}

	/*Get the data*/
	*datapt =  *(rx_get_pt++);

	/*Check if at the end*/
	if( rx_get_pt == &RX_FIFO[RXFIFOSIZE])
	{
		/*Wrap around*/
		rx_get_pt =  &RX_FIFO[0];
	}

	return (RXFIFOSUCCESS);
}


