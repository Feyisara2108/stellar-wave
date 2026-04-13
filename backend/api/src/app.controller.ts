import { Controller, Get, Param, Post, Body, Inject } from '@nestjs/common';
import { PrismaClient } from '@prisma/client';

@Controller()
export class AppController {
  constructor(@Inject('PRISMA_SERVICE') private readonly prisma: PrismaClient) {}

  @Get('streams/:user')
  async getStreamsByUser(@Param('user') user: string) {
    return this.prisma.stream.findMany({
      where: {
        OR: [{ sender: user }, { recipient: user }],
      },
      include: { claims: true }
    });
  }

  @Get('streams/id/:id')
  async getStreamById(@Param('id') id: string) {
    return this.prisma.stream.findUnique({
      where: { id },
      include: { claims: true }
    });
  }

  @Get('claimable/:id')
  async getClaimable(@Param('id') id: string) {
    const stream = await this.prisma.stream.findUnique({ where: { id }});
    if (!stream) return { claimable: 0n };
    
    // Abstracted calculation placeholder (indexer updates vested)
    return { claimable: stream.totalAmount - stream.claimedAmount };
  }

  @Post('simulate-claim')
  async simulateClaim(@Body() body: { streamId: string, recipient: string }) {
    return { success: true, simulatedOutputs: [] };
  }
}
