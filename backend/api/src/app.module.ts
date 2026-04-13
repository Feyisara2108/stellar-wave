import { Module } from '@nestjs/common';
import { AppController } from './app.controller';

/**
 * Prisma service directly configured inline for brevity.
 */
import { PrismaClient } from '@prisma/client';
const prisma = new PrismaClient();

@Module({
  imports: [],
  controllers: [AppController],
  providers: [{ provide: 'PRISMA_SERVICE', useValue: prisma }],
})
export class AppModule {}
